# Security Audit Report — gravity-aptos (Round 4)

**Date:** 2026-03-05
**Scope:** Full gravity-aptos codebase — consensus, DKG, network, event handling, Gravity-specific bridging
**Language:** Rust | **Framework:** AptosBFT (Gravity fork)
**Repository:** `Galxe/gravity-aptos` (branch: `security-audit-fixes`)
**Prior Audits:** [Round 1 — 2026-02-26](./2026-02-26-security-audit-report.md), [Round 2 — 2026-03-01](./2026-03-01-security-audit-report-round2.md), [Round 3 — 2026-03-04](./2026-03-04-security-audit-report-round3.md) (31 findings)

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 2 |
| HIGH | 4 |
| MEDIUM | 6 |
| LOW | 3 |
| INFO | 3 |
| **Total** | **18** |

> [!NOTE]
> This Round 4 audit was performed as part of the cross-repository deep security review.
> It includes regression checks on all prior round findings and cross-module analysis with grevm, gravity-reth, gravity-sdk, and gravity_chain_core_contracts.

---

## CRITICAL (2)

### GAPTOS-R4-001: `GTxnBytes` Payload Causes Node Crash via `todo!()` in 8+ Production Paths

**Files:** `aptos_vm.rs:1981,2592`, `transaction_metadata.rs:69`, `transactions.rs:1110,1428`, `convert.rs:372`, `aptos_debugger.rs:156,394`

The `GTxnBytes` variant of `TransactionPayload` is matched with `todo!()` in at least 8 non-test production code paths. If a `GTxnBytes` transaction reaches VM execution, API simulation, or payload conversion, the node panics. Combined with disabled VM validation in the mempool (GAPTOS-R4-003), this is a direct DoS attack path.

**Prior:** GAPTOS-R2-001. **Still unfixed after 4 audit rounds.**

### GAPTOS-R4-002: `GravityExtension` Fields Not Covered by Transaction Signature

**File:** `types/src/transaction/mod.rs:568-616`

`SignedTransaction` contains `g_ext: GravityExtension` (block_id, txn_index_in_block, txn_count_in_block). The authenticator verifies `raw_txn` which does NOT include `g_ext`. These fields are unsigned metadata freely modifiable after signing — transaction malleability vector.

**Prior:** GAPTOS-R2-005. **Still unfixed after 3 audit rounds.**

---

## HIGH (4)

### GAPTOS-R4-003: VM Validation Completely Disabled in Mempool [BY DESIGN]

Combined with GAPTOS-R4-001 — creates a direct attack path. Any data can enter the mempool without format/signature/gas validation.

**Prior:** P0-E3-1 (Round 3), marked "符合预期." Gravity uses L1-side verification instead of VM-level mempool validation.
**Status:** Accepted risk — by design.

### GAPTOS-R4-004: `from_str().unwrap()` in Config Provider

**File:** `event-notifications/src/lib.rs:426,435`

`OnChainConfig::from_str(T::TYPE_IDENTIFIER).unwrap()` — panics if config type not in enum. Epoch `TryInto::<u64>::try_into(epoch_bytes).unwrap()` at line 316 still unfixed.

**Prior:** P0-B4-1 (Round 3). **Still unfixed.**

### GAPTOS-R4-005: `From<GravityEvent>` Uses `.expect()` on Unknown JWK Types

**File:** `contract_event.rs:545-548`

`TryFrom` correctly returns `Err` for unknown JWK types, but `From` impl wraps with `.expect()` — panics on any unknown JWK.

**Prior:** P0-C3-1, GAPTOS-R2-002. **Still unfixed after 3 rounds.**

### GAPTOS-R4-006: `bcs::to_bytes().unwrap()` in JWK/DKG Event Serialization

**File:** `contract_event.rs:526,538`

Inconsistent with NewEpochEvent branch which uses `.map_err()?`. Externally-sourced JWK data could cause serialization failure → panic.

**Prior:** GAPTOS-R2-003. **Still unfixed.**

---

## MEDIUM (6)

### GAPTOS-R4-007: `DKGState::last_complete()` Panics on Missing State

**File:** `types/src/dkg/mod.rs:429-431`

Returns `.unwrap()` on `last_completed` which is `None` during genesis/first epoch.

### GAPTOS-R4-008: `RoundProposer` Fallback Degrades to Single-Proposer

**File:** `consensus/src/epoch_manager.rs:385-392`

Falls back to `RotatingProposer` with ONLY the first proposer. Discards entire `round_proposers` map. Single point of failure for liveness.

### GAPTOS-R4-009: DKG Epoch Manager `shutdown_current_processor` Double `.unwrap()`

**File:** `dkg/src/epoch_manager.rs:266-269`

Panics if DKG manager task has already failed when epoch manager tries to shut it down.

### GAPTOS-R4-010: DKG `start_new_epoch` Multiple `.expect()` Calls

**File:** `dkg/src/epoch_manager.rs:151,154,160`

Critical startup path. ValidatorSet retrieval uses `.expect()`.

### GAPTOS-R4-011: `state_computer.rs` Mock Executor Has `todo!()`

**File:** `consensus/src/state_computer.rs:557,565,569`

Three methods contain `todo!()`. Need verification they are properly `#[cfg(test)]` gated.

### GAPTOS-R4-012: DKG `InnerState::my_node_cloned()` Panics in NotStarted

**File:** `dkg/src/dkg_manager/mod.rs:82-83`

Panics if transcript requested before DKG session started.

---

## LOW (3)

### GAPTOS-R4-013: Consensus Config BCS Layer Skip (TODO Comment)

Intentional difference but unverified. **Prior:** P1-B2-1.

### GAPTOS-R4-014: Large Amounts of Commented-Out Code

31+ files. **Prior:** P2-B1-1.

### GAPTOS-R4-015: `agg_trx_producer` Uses `.expect()` on Broadcast

**File:** `dkg/src/agg_trx_producer.rs:67,74`

Assumes broadcast infrastructure never fails.

---

## INFO (3)

### GAPTOS-R4-016: Noise Handshake Peer ID Check Restored ✓

Previously removed, now correctly restored.

### GAPTOS-R4-017: BFT Safety Properties Preserved ✓

Core 2f+1 voting, safety rules, equivocation detection intact from upstream Aptos.

### GAPTOS-R4-018: Quorum Store Validation Intact ✓

Batch generation, proof-of-store, quota management unmodified.

---

## Regression Summary

| Prior Finding | Current Status |
|---|---|
| GAPTOS-R2-001 (GTxnBytes todo) | **Still open** (4 rounds) |
| GAPTOS-R2-002 (From\<GravityEvent\> expect) | **Still open** (3 rounds) |
| GAPTOS-R2-003 (bcs unwrap in event serialization) | **Still open** (3 rounds) |
| GAPTOS-R2-005 (GravityExtension unsigned) | **Still open** (3 rounds) |
| P0-B4-1 (config path unwrap) | **Still open** (2 rounds) |
| P0-C3-1 (JWK panic) | **Partially fixed** — TryFrom returns Err but From still uses .expect() |
| P0-D1-1 (Cargo.lock gitignored) | **Fixed** ✓ |
| Noise peer_id removal | **Fixed** ✓ |

---

## Cumulative Statistics (Rounds 1-4)

| Severity | R1 | R2 | R3 | R4 | Total |
|----------|----|----|----|----|-------|
| CRITICAL | 0 | 2 | 0 | 2 | 4 |
| HIGH | 0 | 3 | 4 | 4 | 11 |
| MEDIUM | 0 | 5 | 12 | 6 | 23 |
| LOW | 0 | 3 | 8 | 3 | 14 |
| INFO | 0 | 2 | 7 | 3 | 12 |
| **Total** | **0** | **15** | **31** | **18** | **~64** |

> Note: Round 1 findings were embedded in the initial audit report with a different numbering scheme.
