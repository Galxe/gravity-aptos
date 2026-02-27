# Security Fix Checklist — gravity-aptos

**Audit Date:** 2026-02-26
**Total Findings:** 31 (1 CRITICAL, 5 HIGH, 8 MEDIUM, 9 LOW, 8 INFO)
**Fix Date:** 2026-02-27
**Fix Branch:** `security-audit-fixes`

## CRITICAL

- [x] **GAPTOS-001** — Fix NewEpochEvent serialization: changed `serde_json::to_vec` to `bcs::to_bytes`
  - [x] File: `types/src/contract_event.rs:482` — returns `Err` instead of `unwrap()` on serialization failure

## HIGH

- [x] **GAPTOS-002** — Restored Noise handshake peer_id derivation check for untrusted peers
  - [x] File: `network/framework/src/noise/handshake.rs` — added `from_identity_public_key` verification before assigning role
- [x] **GAPTOS-003** — Restored on-chain `RandomnessConfigSeqNum` read in DKG epoch manager
  - [x] File: `dkg/src/epoch_manager.rs` — replaced hardcoded `{ seq_num: 0 }` with `payload.get::<RandomnessConfigSeqNum>()`
- [x] **GAPTOS-004** — Replaced `.unwrap()` with `?` in `construct_and_convert_validator_set()`
  - [x] File: `types/src/idl/api_types_converter.rs` — proper error propagation via `ValidatorInfoIdlError`
- [x] **GAPTOS-005** — Replaced `.unwrap()` with error propagation in DKG validator set methods
  - [x] File: `types/src/dkg/mod.rs` — return types changed to `Result<Vec<ValidatorConsensusInfo>>`
  - [x] All callers updated in `real_dkg/mod.rs` and `testsuite/smoke-test/src/randomness/mod.rs`
- [x] **GAPTOS-006** — Replaced `panic!()` with `Err` in JWK type conversion
  - [x] File: `types/src/contract_event.rs:517` — returns `Err(anyhow!(...))` for unknown JWK types
  - [x] Replaced `impl Into<ContractEvent>` with proper `impl From<GravityEvent>`

## MEDIUM

- [x] **GAPTOS-007** — Replaced `todo!()` in `RoundProposer` with fallback to `RotatingProposer`
  - [x] File: `consensus/src/epoch_manager.rs:390` — logs warning and falls back to single-proposer rotation
- [ ] **GAPTOS-008** — Document `LedgerInfo` field determinism requirements (deferred: design review needed)
- [ ] **GAPTOS-009** — Add JWK provider garbage collection (deferred: requires design discussion)
- [x] **GAPTOS-010** — Restored JWK sorting for non-gravity sources
  - [x] File: `crates/aptos-jwk-consensus/src/jwk_observer.rs` — only skips sort for `gravity://` prefixed issuers
- [x] **GAPTOS-011** — Fixed X25519 key generation to use provided RNG
  - [x] File: `crates/aptos-crypto/src/x25519.rs` — changed `StaticSecret::random()` to `StaticSecret::new(rng)`
- [x] **GAPTOS-012** — Redacted sensitive data from on-disk storage debug log
  - [x] File: `secure/storage/src/on_disk.rs:64` — logs only key names, not values
- [x] **GAPTOS-013** — Added warning log for network address parse fallback
  - [x] File: `types/src/idl/api_types_converter.rs` — `tracing::warn!` before returning empty vec
- [ ] **GAPTOS-014** — Verify and document consensus config deserialization format (deferred: requires cross-repo verification)

## LOW

- [x] **GAPTOS-015** — Updated `ChainId::FromStr` to parse as `u64`
  - [x] File: `types/src/chain_id.rs:186`
- [ ] **GAPTOS-016** — Add length validation for `reth_account_address` (deferred: requires API contract review)
- [ ] **GAPTOS-017** — Include `g_ext` in `SignedTransaction::PartialEq` (deferred: design decision needed)
- [x] **GAPTOS-018** — Handle `GLOBAL_RELAYER.get()` returning `None` gracefully
  - [x] File: `crates/aptos-jwk-consensus/src/jwk_observer.rs` — error logging and early return instead of panic
- [ ] **GAPTOS-019** — Add `reth_account_address` to `ValidatorInfoIdl` (deferred: IDL format change)
- [x] **GAPTOS-020** — Replaced `eprintln!()` with `tracing::error!()`
  - [x] File: `types/src/contract_event.rs` — structured logging for address and public key parse errors
- [ ] **GAPTOS-021** — Add `ConfigSanitizer` for HTTPS cert/key path validation (deferred: low risk in practice)
- [x] **GAPTOS-022** — Implemented `Bcs::meta()` (replaced `todo!()`)
  - [x] File: `api/src/bcs_payload.rs` — returns proper `MetaResponses`
- [ ] **GAPTOS-023** — Handle malformed type tags in `new_v2_with_type_tag_str` (deferred: all callers use compile-time constants)

## INFO

- [x] **GAPTOS-INFO-005** — Replaced `impl Into<ContractEvent>` with `impl From<GravityEvent>`
  - [x] File: `types/src/contract_event.rs`
- [x] **GAPTOS-INFO-006** — Fixed error variant naming: `BcsDeserializationError` for BCS operations
  - [x] Files: `types/src/idl/error.rs`, `types/src/idl/jwk_converter.rs`
- [ ] **GAPTOS-INFO-001** — Chinese comments (deferred: cosmetic)
- [ ] **GAPTOS-INFO-002** — Commented-out code (deferred: cosmetic)
- [ ] **GAPTOS-INFO-003** — `#![allow(dead_code)]` on DAG module (no action needed)
- [x] **GAPTOS-INFO-004** — DKG smoke-test deterministic seed (correctly feature-gated, no action needed)
- [ ] **GAPTOS-INFO-007** — Relayer trust model documentation (deferred: cosmetic)
- [ ] **GAPTOS-INFO-008** — VFN upstream roles expansion (no action needed: intentional)

---

## Fix Summary

| Severity | Total | Fixed | Deferred |
|----------|-------|-------|----------|
| CRITICAL | 1 | 1 | 0 |
| HIGH | 5 | 5 | 0 |
| MEDIUM | 8 | 5 | 3 |
| LOW | 9 | 4 | 5 |
| INFO | 8 | 3 | 5 |
| **Total** | **31** | **18** | **13** |

## Cross-Repository Concerns

| Finding | Affects | Related |
|---------|---------|---------|
| GAPTOS-001 (serialization mismatch) | gravity-aptos ↔ gravity-sdk | Epoch transitions |
| GAPTOS-003 (DKG config hardcoded) | gravity-aptos ↔ gravity-reth | On-chain config |
| GAPTOS-008 (LedgerInfo fields) | gravity-aptos ↔ gravity-sdk | Consensus signing |
| GAPTOS-014 (config deserialization) | gravity-aptos ↔ gravity-reth | Config format |

## Test Plan

- [ ] Run consensus unit tests: `cargo test -p aptos-consensus`
- [ ] Run DKG tests: `cargo test -p aptos-dkg`
- [ ] Run network handshake tests: `cargo test -p aptos-network`
- [ ] Run type serialization tests: `cargo test -p aptos-types`
- [ ] Run JWK tests: `cargo test -p aptos-jwk-consensus`
- [ ] Run IDL converter tests: `cargo test -p aptos-types -- idl`
- [ ] Verify epoch transitions work end-to-end (e2e smoke test)
