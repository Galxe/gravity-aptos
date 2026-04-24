//! GCP Secret Manager fetch helper for identity blobs.
//!
//! `IdentityBlob` (the validator's YAML bundle of network / consensus / account
//! keys) is normally loaded from disk. For deployments that want to avoid
//! materializing private keys on the filesystem, the same bytes can instead be
//! stored as a Secret Manager payload and pulled at startup.
//!
//! ## Feature flag
//!
//! The HTTP client (`reqwest`) and `base64` dependencies are gated behind the
//! `gcp-secret-manager` Cargo feature so that binaries which never consult
//! Secret Manager do not pay the dependency cost. With the feature off,
//! [`fetch_secret`] still exists but returns an error at runtime pointing the
//! operator at the missing flag. Downstream enum variants
//! ([`super::Identity::FromGcpSecret`] and
//! [`super::InitialSafetyRulesConfig::FromGcpSecret`]) remain unconditional so
//! match sites compile identically regardless of feature state.
//!
//! ## Auth
//!
//! - On a GCE VM: no extra config — the VM's bound service account token is
//!   fetched from the metadata server.
//! - Anywhere else: set `GCP_ACCESS_TOKEN` (e.g. `export
//!   GCP_ACCESS_TOKEN=$(gcloud auth print-access-token)`). This escape hatch
//!   keeps the `aptos-config` crate free of the full google-cloud-auth / ADC
//!   dependency tree while remaining usable for local dev.
//!
//! ## Caching
//!
//! Fetched payloads are cached per normalized resource for the lifetime of the
//! process. `NetworkConfig::identity_key` and `NetworkConfig::peer_id` each
//! dereference `Identity::FromGcpSecret` independently, and a validator has
//! both a validator_network and one-or-more full_node_networks — without the
//! cache a single startup would issue N×(metadata_token + secret_access) HTTP
//! calls for the same payload.
//!
//! The secret payload is expected to be the raw YAML bytes of an
//! `IdentityBlob`, exactly as the on-disk file would be.

use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

/// Accept the short form `projects/P/secrets/S` and append the default
/// `versions/latest`, mirroring the KMS signer's resource normalization.
pub fn normalize_secret_resource(s: &str) -> String {
    let trimmed = s.trim().trim_end_matches('/');
    if trimmed.contains("/versions/") {
        trimmed.to_string()
    } else {
        format!("{trimmed}/versions/latest")
    }
}

static CACHE: OnceLock<Mutex<HashMap<String, Vec<u8>>>> = OnceLock::new();

fn cache() -> &'static Mutex<HashMap<String, Vec<u8>>> {
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Fetch the payload of a Secret Manager secret version.
///
/// `resource` accepts the full `projects/<P>/secrets/<S>/versions/<V>` path, or
/// the short form without `/versions/<V>` (defaults to `latest`). Returns the
/// raw decoded bytes of the secret payload. Subsequent calls for the same
/// resource within the same process return the cached bytes.
pub fn fetch_secret(resource: &str) -> anyhow::Result<Vec<u8>> {
    let normalized = normalize_secret_resource(resource);
    if let Ok(guard) = cache().lock() {
        if let Some(bytes) = guard.get(&normalized) {
            return Ok(bytes.clone());
        }
    }
    let bytes = fetch_uncached(&normalized)?;
    if let Ok(mut guard) = cache().lock() {
        guard.insert(normalized, bytes.clone());
    }
    Ok(bytes)
}

#[cfg(not(feature = "gcp-secret-manager"))]
fn fetch_uncached(_resource: &str) -> anyhow::Result<Vec<u8>> {
    anyhow::bail!(
        "GCP Secret Manager support not compiled in. Rebuild aptos-config with \
         `--features gcp-secret-manager` (or enable the forwarding feature on \
         the downstream crate, e.g. `gaptos/gcp-secret-manager`)."
    )
}

#[cfg(feature = "gcp-secret-manager")]
use http::fetch_uncached;

#[cfg(feature = "gcp-secret-manager")]
mod http {
    use anyhow::{anyhow, bail, Context};
    use std::time::Duration;

    const METADATA_TOKEN_URL: &str =
        "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";
    const SECRET_MANAGER_HOST: &str = "https://secretmanager.googleapis.com";
    const TOKEN_ENV: &str = "GCP_ACCESS_TOKEN";
    const HTTP_TIMEOUT: Duration = Duration::from_secs(15);

    pub(super) fn fetch_uncached(resource: &str) -> anyhow::Result<Vec<u8>> {
        let token = access_token()?;

        let url = format!("{SECRET_MANAGER_HOST}/v1/{resource}:access");
        let client = reqwest::blocking::Client::builder()
            .timeout(HTTP_TIMEOUT)
            .build()
            .context("build reqwest client")?;
        let resp = client
            .get(&url)
            .bearer_auth(token)
            .send()
            .with_context(|| format!("GET {url}"))?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().unwrap_or_default();
            bail!("Secret Manager access failed ({status}) for {resource}: {body}");
        }

        let body: AccessResponse = resp.json().context("decode Secret Manager response")?;
        let b64 = body
            .payload
            .ok_or_else(|| anyhow!("Secret Manager response missing payload"))?
            .data;
        base64::decode(b64.as_bytes()).context("decode secret payload base64")
    }

    fn access_token() -> anyhow::Result<String> {
        if let Ok(t) = std::env::var(TOKEN_ENV) {
            let t = t.trim().to_string();
            if !t.is_empty() {
                return Ok(t);
            }
        }
        metadata_token()
    }

    fn metadata_token() -> anyhow::Result<String> {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .context("build metadata client")?;
        let resp = client
            .get(METADATA_TOKEN_URL)
            .header("Metadata-Flavor", "Google")
            .send()
            .map_err(|e| {
                anyhow!(
                    "GCE metadata server unreachable ({e}) — set {TOKEN_ENV} for non-GCE \
                     environments, e.g. `export {TOKEN_ENV}=$(gcloud auth print-access-token)`"
                )
            })?
            .error_for_status()
            .context("metadata server returned non-2xx")?;
        let token: MetadataToken = resp.json().context("decode metadata token response")?;
        Ok(token.access_token)
    }

    #[derive(serde::Deserialize)]
    struct AccessResponse {
        payload: Option<SecretPayload>,
    }

    #[derive(serde::Deserialize)]
    struct SecretPayload {
        data: String,
    }

    #[derive(serde::Deserialize)]
    struct MetadataToken {
        access_token: String,
    }
}

#[cfg(test)]
mod tests {
    use super::normalize_secret_resource;

    #[test]
    fn normalize_appends_default_version() {
        assert_eq!(
            normalize_secret_resource("projects/p/secrets/s"),
            "projects/p/secrets/s/versions/latest"
        );
    }

    #[test]
    fn normalize_keeps_explicit_version() {
        let in_ = "projects/p/secrets/s/versions/3";
        assert_eq!(normalize_secret_resource(in_), in_);
    }

    #[test]
    fn normalize_strips_trailing_slash() {
        assert_eq!(
            normalize_secret_resource("projects/p/secrets/s/"),
            "projects/p/secrets/s/versions/latest"
        );
    }
}
