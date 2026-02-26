# Security Fix Checklist — gravity-aptos

**Audit Date:** 2026-02-26
**Total Findings:** 31 (1 CRITICAL, 5 HIGH, 8 MEDIUM, 9 LOW, 8 INFO)

## CRITICAL

- [ ] **GAPTOS-001** — Fix NewEpochEvent serialization: change `serde_json::to_vec` to `bcs::to_bytes`
  - [ ] File: `types/src/contract_event.rs:482`
  - [ ] Verify deserialization in `NewEpochEvent::try_from_bytes()` matches
  - [ ] Add round-trip test for NewEpochEvent serialization

## HIGH

- [ ] **GAPTOS-002** — Restore Noise handshake peer_id derivation check
  - [ ] File: `network/framework/src/noise/handshake.rs:390-411`
  - [ ] Verify existing test `test_handshake_client_peerid_mismatch_fails_server_only_auth` passes
  - [ ] Document if intentional relaxation is needed for specific use cases
- [ ] **GAPTOS-003** — Restore on-chain `RandomnessConfigSeqNum` read in DKG epoch manager
  - [ ] File: `dkg/src/epoch_manager.rs:170-174`
  - [ ] Remove hardcoded `{ seq_num: 0 }` and TODO comment
  - [ ] Verify greth provides this config value correctly
- [ ] **GAPTOS-004** — Replace `.unwrap()` with `?` in `construct_and_convert_validator_set()`
  - [ ] File: `types/src/idl/api_types_converter.rs:27-30`
  - [ ] Adjust error type if needed
- [ ] **GAPTOS-005** — Replace `.unwrap()` with error propagation in DKG validator set methods
  - [ ] File: `types/src/dkg/mod.rs:92-106`
  - [ ] Change return types to `Result<Vec<ValidatorConsensusInfo>>`
  - [ ] Update all callers
- [ ] **GAPTOS-006** — Replace `panic!()` with `Err` in JWK type conversion
  - [ ] File: `types/src/contract_event.rs:517`
  - [ ] Remove `impl Into<ContractEvent> for GravityEvent` (line 546-549) that wraps with `.unwrap()`

## MEDIUM

- [ ] **GAPTOS-007** — Replace `todo!()` in `RoundProposer` election type
  - [ ] File: `consensus/src/epoch_manager.rs:390`
  - [ ] Return error or use fallback election type
- [ ] **GAPTOS-008** — Document `LedgerInfo` field determinism requirements
  - [ ] File: `types/src/ledger_info.rs:52-62`
  - [ ] Add doc comments explaining when `block_hash`/`block_number` must be set
  - [ ] Consider whether fields should be excluded from hash
- [ ] **GAPTOS-009** — Add JWK provider garbage collection
  - [ ] File: `crates/aptos-jwk-consensus/src/jwk_manager/mod.rs`
  - [ ] Remove JWK entries when providers are removed from on-chain config
- [ ] **GAPTOS-010** — Restore JWK sorting for non-gravity sources
  - [ ] File: `crates/aptos-jwk-consensus/src/jwk_observer.rs:95-97`
  - [ ] Only skip sort for `gravity://` prefixed issuers
  - [ ] Add `debug_assert!` for sorted order on gravity sources
- [ ] **GAPTOS-011** — Fix X25519 key generation to honor RNG parameter
  - [ ] File: `crates/aptos-crypto/src/x25519.rs:177-184`
  - [ ] Use `StaticSecret::random_from_rng(rng)`
- [ ] **GAPTOS-012** — Redact sensitive data from on-disk storage debug log
  - [ ] File: `secure/storage/src/on_disk.rs:64`
  - [ ] Log only key names, not values
- [ ] **GAPTOS-013** — Return error instead of empty vec in `parse_network_address()`
  - [ ] File: `types/src/idl/api_types_converter.rs:103-122`
  - [ ] Add warning log for fallback case
- [ ] **GAPTOS-014** — Verify and document consensus config deserialization format
  - [ ] File: `types/src/on_chain_config/consensus_config.rs:423-429`
  - [ ] Fix stale test on line 614
  - [ ] Remove TODO comment once verified

## LOW

- [ ] **GAPTOS-015** — Update `ChainId::FromStr` to parse as `u64`
- [ ] **GAPTOS-016** — Add length validation for `reth_account_address` (enforce 20 bytes)
- [ ] **GAPTOS-017** — Include `g_ext` in `SignedTransaction::PartialEq` or document exclusion
- [ ] **GAPTOS-018** — Handle `GLOBAL_RELAYER.get()` returning `None` gracefully
- [ ] **GAPTOS-019** — Add `reth_account_address` to `ValidatorInfoIdl` for round-trip fidelity
- [ ] **GAPTOS-020** — Replace `eprintln!()` with `tracing::error!()`
- [ ] **GAPTOS-021** — Add `ConfigSanitizer` for HTTPS cert/key path validation
- [ ] **GAPTOS-022** — Implement or stub `Bcs::meta()` (replace `todo!()`)
- [ ] **GAPTOS-023** — Handle malformed type tags in `new_v2_with_type_tag_str`

## INFO (No action required)

- [x] **GAPTOS-INFO-001** — Chinese comments (translate or remove)
- [x] **GAPTOS-INFO-002** — Commented-out code (remove)
- [x] **GAPTOS-INFO-003** — `#![allow(dead_code)]` on DAG module
- [x] **GAPTOS-INFO-004** — DKG smoke-test deterministic seed (correctly feature-gated)
- [x] **GAPTOS-INFO-005** — `Into` vs `From` convention
- [x] **GAPTOS-INFO-006** — Error variant naming (`JsonDeserializationError` for BCS)
- [x] **GAPTOS-INFO-007** — Relayer trust model documentation
- [x] **GAPTOS-INFO-008** — VFN upstream roles expansion

---

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
