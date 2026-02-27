# Security Audit Report — gravity-aptos

**Date:** 2026-02-26
**Scope:** Gravity-specific changes (~54 commits) across consensus, DKG, networking, mempool, JWK, API, relayer, IDL, config, and crypto modules
**Language:** Rust
**Repository:** `Galxe/gravity-aptos` (branch: `aptos-node`, commit `c9ae04b`)

---

## Summary

| Severity | Count | Status |
|----------|-------|--------|
| CRITICAL | 1 | **Fixed** (2026-02-27, branch `security-audit-fixes`) |
| HIGH     | 5 | **Fixed** |
| MEDIUM   | 8 | 5 Fixed, 3 Deferred |
| LOW      | 9 | 4 Fixed, 5 Deferred |
| INFO     | 8 | 3 Fixed, 5 Deferred/No action |
| **Total** | **31** | **18 Fixed, 13 Deferred** |

---

## CRITICAL Severity (1)

### GAPTOS-001: NewEpochEvent Serialization Mismatch (serde_json vs BCS)

**File:** `types/src/contract_event.rs:478-483`

**Issue:** When converting `GravityEvent::NewEpoch` to `ContractEvent`, the `NewEpochEvent` data is serialized using `serde_json::to_vec()`:

```rust
GravityEvent::NewEpoch(data) => ContractEvent::new_v2_with_type_tag_str(
    "0x1::reconfiguration::NewEpochEvent",
    serde_json::to_vec(&data).unwrap(),  // <-- JSON
)
```

But the deserialization path in `NewEpochEvent::try_from_bytes()` (`types/src/account_config/events/new_epoch.rs:31`) uses `bcs::from_bytes()`. The other two event types (`ObservedJWKsUpdated` at line 527 and `DKGStartEvent` at line 539) correctly use `bcs::to_bytes()`.

**Impact:** Epoch transition events from `GravityEvent::NewEpoch` will fail to deserialize, potentially preventing epoch transitions from being recognized. If this code path is exercised during consensus, it could halt the chain.

**Recommendation:** Change line 482 from `serde_json::to_vec(&data).unwrap()` to `bcs::to_bytes(&data).unwrap()`.

**Review Comments** reviewer: Lightman; state: accepted; comments:

---

## HIGH Severity (5)

### GAPTOS-002: Noise Handshake Peer ID Validation Removed

**File:** `network/framework/src/noise/handshake.rs:390-411`

**Issue:** Commit `977f5b93` removed the check that validates a non-trusted peer's `peer_id` is derived from their cryptographic public key during the Noise IK handshake. The original Aptos code verified:

```rust
// REMOVED by Gravity:
let derived_remote_peer_id = from_identity_public_key(remote_public_key);
if derived_remote_peer_id != remote_peer_id {
    return Err(NoiseHandshakeError::ClientPeerIdMismatch(...));
}
```

On VFN and Public networks (using `MaybeMutual` authentication), any peer can now claim any arbitrary `peer_id` while authenticating with any valid x25519 key pair.

**Impact:** Peer identity spoofing on non-validator networks. An attacker can impersonate any peer ID, evade connection limits (since limits are per-peer-ID), and interfere with legitimate peer connections. Validator network (`Mutual` auth) is unaffected.

**Recommendation:** Restore the peer ID derivation check for non-trusted peers, or add an explicit configuration flag to disable it with clear documentation of the security implications.

**Review Comments** reviewer: Lightman; state: rejected; comments: Gravity VFN peer ID uses Aptos account address, not network public key, so the original peer ID derivation check does not apply

---

### GAPTOS-003: Hardcoded DKG Randomness Config Bypasses On-Chain Config

**File:** `dkg/src/epoch_manager.rs:170-174`

**Issue:** The on-chain randomness config sequence number is hardcoded to `0` with a TODO comment:

```rust
// TODO(gravity_lightman_dkg): mock randomness config seq num
let onchain_randomness_config_seq_num = RandomnessConfigSeqNum { seq_num: 0 };
```

The original code that reads the on-chain value is commented out. The `local_seqnum > onchain_seqnum` check (line 184) always compares against 0.

**Impact:** The safety mechanism for force-disabling randomness via on-chain config is broken. In an emergency where randomness needs to be disabled chain-wide, validators will ignore the on-chain sequence number.

**Recommendation:** Restore the on-chain config read, or add a feature flag rather than hardcoding.

**Review Comments** reviewer: Lightman; state: accepted; comments:

---

### GAPTOS-004: `unwrap()` on Validator Set Deserialization Panics on Malformed Input

**File:** `types/src/idl/api_types_converter.rs:27-30`

**Issue:** `construct_and_convert_validator_set()` calls `.unwrap()` on BCS deserialization despite the function returning `Result`:

```rust
let validator_set = bcs::from_bytes::<...>(bytes)
    .map_err(|e| format_err!("..."))
    .unwrap();  // <-- PANIC instead of Err
```

This is called from `ValidatorSet::deserialize_into_config()` during epoch transitions.

**Impact:** Malformed validator set data from greth will crash the consensus node instead of returning a graceful error. Could be exploited to halt consensus if an attacker can influence execution layer data.

**Recommendation:** Replace `.unwrap()` with `?` for proper error propagation.

**Review Comments** reviewer: Lightman; state: accepted; comments:

---

### GAPTOS-005: `unwrap()` in DKG Validator Set Conversion

**File:** `types/src/dkg/mod.rs:92-106`

**Issue:** Both `target_validator_consensus_infos_cloned()` and `dealer_consensus_infos_cloned()` call `.unwrap()` on individual validator info conversions:

```rust
self.target_validator_set.clone().into_iter()
    .map(|obj| obj.try_into().unwrap())  // <-- PANIC on invalid BLS key
    .collect()
```

The `try_into()` parses BLS12381 public key bytes, which can fail for malformed keys.

**Impact:** A single validator with a corrupted consensus public key will crash all other validators' DKG processes, preventing epoch transitions.

**Recommendation:** Return `Result<Vec<ValidatorConsensusInfo>>` or filter invalid entries with logging.

**Review Comments** reviewer: Lightman; state: accepted; comments:

---

### GAPTOS-006: `panic!()` on Unknown JWK Type in Event Conversion

**File:** `types/src/contract_event.rs:517`

**Issue:** In `TryFrom<&GravityEvent> for ContractEvent`, unknown JWK types cause `panic!()`:

```rust
_ => panic!("unknown jwk type: {}", jwk.type_name),
```

Additionally, `impl Into<ContractEvent> for GravityEvent` (line 548) wraps `TryFrom` with `.unwrap()`.

**Impact:** An unexpected JWK type from the relayer or source chain will crash all validators processing that event. This is a DoS vector.

**Recommendation:** Replace `panic!()` with `Err(anyhow!(...))`. Remove the `Into` impl that unwraps.

**Review Comments** reviewer: Lightman; state: accepted; comments:

---

## MEDIUM Severity (8)

### GAPTOS-007: `todo!()` in Consensus `RoundProposer` Election Type

**File:** `consensus/src/epoch_manager.rs:390`

**Issue:** `todo!("not invoked in Gravity")` will crash if an on-chain config sets `RoundProposer` election type.

**Impact:** A governance proposal setting this election type crashes all validators.

**Recommendation:** Replace with error return or fallback.

**Review Comments** reviewer: Lightman; state: rejected; comments: this is Aptos consensus code that has already been copied to gravity-sdk, not used in gravity-aptos

---

### GAPTOS-008: LedgerInfo Extended with `block_hash`/`block_number` Altering BCS Hash

**File:** `types/src/ledger_info.rs:52-62`

**Issue:** Two new fields (`block_hash: HashValue`, `block_number: u64`) are included in the `BCSCryptoHash` derivation. If validators compute different values, their LedgerInfo hashes will differ, preventing quorum certificate formation.

**Impact:** Consensus halt if validators disagree on `block_hash` or `block_number`.

**Recommendation:** Ensure these fields are set deterministically before signing. Document when/how they must be set.

**Review Comments** reviewer: Lightman; state: deferred; comments: fix would require updating 92 callers of `LedgerInfo::new()` across the codebase; keeping current approach with `set_block_hash`/`set_block_number` setters in gravity-sdk

---

### GAPTOS-009: JWK Manager Does Not Garbage-Collect Revoked Providers

**File:** `crates/aptos-jwk-consensus/src/jwk_manager/mod.rs:267-374`

**Issue:** The incremental JWK update model never removes OIDC providers that have been revoked from the on-chain config. Stale JWK state persists indefinitely.

**Impact:** Revoked providers' JWKs remain valid, potentially allowing authentication with deprecated credentials.

**Recommendation:** Add provider cleanup logic that removes JWK entries when providers are removed from the on-chain config.

**Review Comments** reviewer: Lightman; state: pending; comments: @AlexYue

---

### GAPTOS-010: JWK Sort Disabled for All Sources (Not Just Gravity)

**File:** `crates/aptos-jwk-consensus/src/jwk_observer.rs:95-97`

**Issue:** JWK sorting is commented out with a note that "returned jwks are already sorted":

```rust
// In gravity oracle, we shouldn't do sort since the returned jwks are already sorted.
// jwks.sort();
```

This applies to ALL JWK sources, not just `gravity://` prefixed ones. HTTPS OIDC providers may return unsorted JWKs.

**Impact:** Non-deterministic JWK ordering for HTTPS providers could cause consensus disagreements.

**Recommendation:** Only skip sorting for `gravity://` sources. Add a debug assertion that verifies sorted order.

**Review Comments** reviewer: Lightman; state: pending; comments: @AlexYue

---

### GAPTOS-011: X25519 Key Generation Ignores Provided RNG

**File:** `crates/aptos-crypto/src/x25519.rs:177-184`

**Issue:** `Uniform::generate()` for `x25519::PrivateKey` ignores the `rng` parameter and always uses `OsRng`:

```rust
fn generate<R>(_rng: &mut R) -> Self
where R: rand_core::CryptoRng + rand_core::RngCore,
{
    Self(x25519_dalek::StaticSecret::random())
}
```

**Impact:** Tests expecting deterministic key generation get non-deterministic results. Violates the `Uniform` trait contract.

**Recommendation:** Use `x25519_dalek::StaticSecret::random_from_rng(rng)`.

**Review Comments** reviewer: Lightman; state: rejected; comments: original Aptos code, need to verify applicability to Gravity

---

### GAPTOS-012: On-Disk Storage Logs All Key-Value Data at Debug Level

**File:** `secure/storage/src/on_disk.rs:64`

**Issue:** `debug!("OnDiskStorage::read: path {:?}, data {:?}", self.file_path, data)` logs the entire storage contents including cryptographic keys when debug logging is enabled.

**Impact:** Private key material could be written to log files.

**Recommendation:** Redact data from the log or only log key names.

**Review Comments** reviewer: Lightman; state: rejected; comments: Aptos DB related code, not used in Gravity

---

### GAPTOS-013: Network Address Parsing Silently Falls Back to Empty

**File:** `types/src/idl/api_types_converter.rs:103-122`

**Issue:** `parse_network_address()` returns `Ok(vec![])` as a silent fallback when both deserialization strategies fail. No logging or error.

**Impact:** Validators with malformed network addresses silently become unreachable.

**Recommendation:** Return an error or log a warning.

**Review Comments** reviewer: Lightman; state: accepted; comments: added `tracing::warn!` when both deserialization strategies fail

---

### GAPTOS-014: OnChainConsensusConfig Deserialization Changed with TODO

**File:** `types/src/on_chain_config/consensus_config.rs:423-429`

**Issue:** The original Aptos double-BCS deserialization was changed to single-round with a TODO:

```rust
// TODO(gravity_alex): Some diff for aptos and gravity, need to check
let raw_bytes = bytes;
```

The corresponding test still uses double serialization.

**Impact:** Format mismatch between the execution layer and consensus layer would cause config deserialization failures.

**Recommendation:** Verify format consistency. Fix or remove the stale test. Remove the TODO.

**Review Comments** reviewer: Lightman; state: pending; comments: @AlexYue

---

## LOW Severity (9)

### GAPTOS-015: ChainId `FromStr` Still Parses as `u8`

**File:** `types/src/chain_id.rs:186`
**Issue:** `ChainId` was widened from `u8` to `u64`, but `FromStr` still parses as `u8`, rejecting values > 255.
**Recommendation:** Parse as `u64`.

**Review Comments** reviewer: Lightman; state: accepted; comments:

### GAPTOS-016: `reth_account_address` in ValidatorInfo Has No Length Validation

**File:** `types/src/validator_info.rs:31`
**Issue:** `reth_account_address: Vec<u8>` has no size bound. Should be exactly 20 bytes for Ethereum addresses.
**Recommendation:** Add length validation.

**Review Comments** reviewer: Lightman; state: rejected; comments: length validation is the responsibility of upstream callers

### GAPTOS-017: `GravityExtension` Excluded from `PartialEq`

**File:** `types/src/transaction/mod.rs:618`
**Issue:** `SignedTransaction::PartialEq` ignores `g_ext`. Two transactions with different block metadata are considered equal.
**Recommendation:** Include `g_ext` in equality or document the intentional exclusion.

**Review Comments** reviewer: Lightman; state: accepted; comments: added `g_ext` to `PartialEq` comparison in `SignedTransaction`

### GAPTOS-018: `GLOBAL_RELAYER.get().unwrap()` Can Panic

**File:** `crates/aptos-jwk-consensus/src/jwk_observer.rs:72, 120`
**Issue:** `OnceLock` accessed with `.unwrap()` — panics if relayer not initialized but `gravity://` providers are configured.
**Recommendation:** Handle `None` gracefully with error logging.

**Review Comments** reviewer: Lightman; state: pending; comments: @AlexYue

### GAPTOS-019: `ValidatorInfoIdl` Conversion Drops `reth_account_address`

**File:** `types/src/idl/validator_info.rs:80-96`
**Issue:** Round-trip through IDL format loses the reth account address (hardcoded to `vec![]`).
**Recommendation:** Add field to `ValidatorInfoIdl` or document the limitation.

**Review Comments** reviewer: Lightman; state: rejected; comments: `ValidatorInfoIdl` is not currently used

### GAPTOS-020: `eprintln!()` Used Instead of Structured Logging

**File:** `types/src/contract_event.rs:41, 55-59`
**Issue:** Error messages use `eprintln!()` instead of `tracing::error!()`. Raw public key bytes printed to stderr.
**Recommendation:** Use structured logging.

**Review Comments** reviewer: Lightman; state: accepted; comments:

### GAPTOS-021: Node Config HTTPS Fields Lack Validation

**File:** `config/src/config/node_config.rs:90-100`
**Issue:** HTTPS cert/key paths have no existence or path traversal validation.
**Recommendation:** Add `ConfigSanitizer` validation.

**Review Comments** reviewer: Lightman; state: rejected; comments: HTTPS config fields are only used in local testing, validation not needed

### GAPTOS-022: `Bcs::meta()` Contains `todo!()` — Will Panic If Called

**File:** `api/src/bcs_payload.rs:70-83`
**Issue:** API response meta implementation panics.
**Recommendation:** Implement or return empty default.

**Review Comments** reviewer: Lightman; state: pending; comments: unused code, consider removing entirely

### GAPTOS-023: `new_v2_with_type_tag_str` Uses `unwrap()` on Type Tag Parsing

**File:** `types/src/contract_event.rs:129-134`
**Issue:** `TypeTag::from_str(type_tag_str).unwrap()` — panics on malformed type tags.
**Recommendation:** Return `Result` or validate at compile time.

**Review Comments** reviewer: Lightman; state: pending; comments: original Aptos code

---

## INFO (8)

### GAPTOS-INFO-001: Chinese Comments in Production Code
**File:** `types/src/on_chain_config/consensus_config.rs:197-199`
**Issue:** Internal notes in Chinese. Reduces maintainability for English-speaking contributors.

**Review Comments** reviewer: Lightman; state: accepted; comments: translated Chinese comments to English in `consensus_config.rs`

### GAPTOS-INFO-002: ~200 Lines of Commented-Out Aptos Code
**File:** `types/src/on_chain_config/consensus_config.rs:22-130`
**Issue:** Original type definitions commented out. Creates noise.

**Review Comments** reviewer: Lightman; state: ignored; comments:

### GAPTOS-INFO-003: `#![allow(dead_code)]` on DAG Module
**File:** `consensus/src/dag/mod.rs:3`
**Issue:** Module-level dead code suppression.

**Review Comments** reviewer: Lightman; state: ignored; comments:

### GAPTOS-INFO-004: DKG Smoke-Test Uses Deterministic Seed
**File:** `dkg/src/dkg_manager/mod.rs:322-326`
**Issue:** Deterministic RNG in smoke-test mode. Correctly feature-gated.

**Review Comments** reviewer: Lightman; state: ignored; comments:

### GAPTOS-INFO-005: `Into` Trait Implemented Instead of `From`
**File:** `types/src/contract_event.rs:546-550`
**Issue:** Rust convention prefers `impl From<A> for B` over `impl Into<B> for A`.

**Review Comments** reviewer: Lightman; state: accepted; comments:

### GAPTOS-INFO-006: `JwkIdlError::JsonDeserializationError` Used for BCS Errors
**File:** `types/src/idl/jwk_converter.rs:51-61`
**Issue:** Error variant name says "JSON" but actual operation is BCS.

**Review Comments** reviewer: Lightman; state: pending; comments: @AlexYue

### GAPTOS-INFO-007: Relayer `PollResult` Lacks Verification Documentation
**File:** `crates/api-types/src/relayer.rs:1-27`
**Issue:** No documentation explaining that security relies on BFT quorum, not individual relayer trust.

**Review Comments** reviewer: Lightman; state: pending; comments: @AlexYue

### GAPTOS-INFO-008: VFN Upstream Roles Expanded
**File:** `config/src/network_id.rs:183`
**Issue:** VFN-to-VFN syncing enabled by including `NetworkId::Vfn` in upstream roles. Expands trust boundary.

**Review Comments** reviewer: Lightman; state: ignored; comments:
