//! GCP Secret Manager fetch helper for identity blobs.
//!
//! `IdentityBlob` (the validator's YAML bundle of network / consensus / account
//! keys) is normally loaded from disk. For deployments that want to avoid
//! materializing private keys on the filesystem, the same bytes can instead be
//! stored as a Secret Manager payload and pulled at startup.
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
//! The secret payload is expected to be the raw YAML bytes of an
//! `IdentityBlob`, exactly as the on-disk file would be.

use anyhow::{anyhow, bail, Context};
use std::time::Duration;

const METADATA_TOKEN_URL: &str =
    "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";
const SECRET_MANAGER_HOST: &str = "https://secretmanager.googleapis.com";
const TOKEN_ENV: &str = "GCP_ACCESS_TOKEN";
const HTTP_TIMEOUT: Duration = Duration::from_secs(15);

/// Fetch the payload of a Secret Manager secret version.
///
/// `resource` must be `projects/<P>/secrets/<S>/versions/<V>`; the trailing
/// `/versions/<V>` may be omitted and defaults to `latest`. Returns the raw
/// decoded bytes of the secret payload.
pub fn fetch_secret(resource: &str) -> anyhow::Result<Vec<u8>> {
    let resource = normalize_secret_resource(resource);
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
        bail!("Secret Manager access failed ({status}): {body}");
    }

    let body: AccessResponse = resp.json().context("decode Secret Manager response")?;
    let b64 = body
        .payload
        .ok_or_else(|| anyhow!("Secret Manager response missing payload"))?
        .data;
    base64::decode(b64.as_bytes()).context("decode secret payload base64")
}

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
