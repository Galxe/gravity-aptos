# Gravity-Aptos Security Audit Report — Round 2

**Date:** 2026-03-01  
**Branch:** `security-audit-fixes-round2`  
**Scope:** Gravity-specific code paths in `gravity-aptos` (types, VM, API, consensus, execution layers)  
**Methodology:** Manual static analysis + pattern-based scanning (`todo!()`, `.unwrap()`, `.expect()`, `unsafe`, Gravity-specific patterns)

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 1     |
| HIGH     | 2     |
| MEDIUM   | 3     |
| LOW      | 1     |
| INFO     | 1     |
| **Total** | **8** |

---

## Findings

### GAPTOS-R2-001 — `GTxnBytes` Payload Causes Runtime Panic via `todo!()`

**Severity:** CRITICAL  
**Status:** Open

**Description:**

The `GTxnBytes` variant of `TransactionPayload` is matched with `todo!()` in **8 production code sites** across the codebase. If a `GTxnBytes` transaction reaches any of these paths, the node will panic and crash immediately. This is a denial-of-service vector: a malicious or malformed transaction with this payload type will bring down any node that attempts to process, simulate, convert, or meter it.

**Affected Files:**

| File | Line | Context |
|------|------|---------|
| [aptos_vm.rs](file:///home/neko/gravity-aptos/aptos-move/aptos-vm/src/aptos_vm.rs) | ~1981 | `execute_user_transaction_impl` |
| [transactions.rs](file:///home/neko/gravity-aptos/api/src/transactions.rs) | ~1110 | `get_signed_transaction` |
| [transactions.rs](file:///home/neko/gravity-aptos/api/src/transactions.rs) | ~1428 | `simulate` |
| [convert.rs](file:///home/neko/gravity-aptos/api/types/src/convert.rs) | 372 | `try_into_transaction_payload` |
| [transaction_metadata.rs](file:///home/neko/gravity-aptos/aptos-move/aptos-vm/src/transaction_metadata.rs) | 69 | `TransactionMetadata::new` — `script_hash` computation |
| [metrics.rs](file:///home/neko/gravity-aptos/execution/executor/src/metrics.rs) | 484 | `update_counters_for_processed_chunk` |
| [aptos_debugger.rs](file:///home/neko/gravity-aptos/aptos-move/aptos-debugger/src/aptos_debugger.rs) | — | Debugger replay path |

**Impact:**

- **Availability:** Any node processing a `GTxnBytes` transaction will crash (DoS).
- **Consensus:** If such a transaction enters the mempool and reaches block execution, it will crash all validators.
- **API:** The REST API will crash if a client submits or simulates a `GTxnBytes` transaction.

**Recommendation:**

Either (a) implement proper handling for `GTxnBytes` across all match arms, or (b) reject `GTxnBytes` transactions early in the mempool/API validation layer with a graceful error (return `Err(...)` or `TransactionStatus::Discard` instead of `todo!()`).

---

### GAPTOS-R2-002 — `From<GravityEvent>` Panics on Conversion Failure

**Severity:** HIGH  
**Status:** Open

**Description:**

In [contract_event.rs:545-549](file:///home/neko/gravity-aptos/types/src/contract_event.rs#L545-L549):

```rust
impl From<GravityEvent> for ContractEvent {
    fn from(event: GravityEvent) -> ContractEvent {
        ContractEvent::try_from(&event).expect("GravityEvent to ContractEvent conversion failed")
    }
}
```

The `From` impl calls `.expect()`, which will panic if the `TryFrom` conversion fails. The `TryFrom` conversion can fail for unknown JWK types (line 516: `Err(anyhow!("unknown jwk type: ..."))`). If an unexpected JWK type arrives from the consensus layer, the node crashes.

**Impact:**

- **Availability:** Unknown or malformed `GravityEvent` variants will crash the node.
- **Chain of causation:** This `From` impl is the idiomatic conversion path, used anywhere `.into()` is called on a `GravityEvent`.

**Recommendation:**

Remove the `From` impl and require callers to use `TryFrom` explicitly, handling the error gracefully. Alternatively, log the error and return a sentinel/default event if crash-safety is preferred.

---

### GAPTOS-R2-003 — `bcs::to_bytes().unwrap()` in Event Serialization

**Severity:** HIGH  
**Status:** Open

**Description:**

In [contract_event.rs:526](file:///home/neko/gravity-aptos/types/src/contract_event.rs#L526) and [contract_event.rs:538](file:///home/neko/gravity-aptos/types/src/contract_event.rs#L538):

```rust
// ObservedJWKsUpdated event (line 526)
bcs::to_bytes(&data).unwrap(),

// DKG event (line 538)
bcs::to_bytes(&data).unwrap(),
```

These are inside `TryFrom<&GravityEvent> for ContractEvent`. While `NewEpochEvent` serialization at line 481 correctly uses `.map_err()`/`?`, the `ObservedJWKsUpdated` and `DKG` branches use `.unwrap()`, creating an inconsistent error-handling pattern.

**Impact:**

If BCS serialization fails (e.g., oversized data, corrupted intermediate state), the node panics. This is especially risky for `ObservedJWKsUpdated` which processes external JWK data.

**Recommendation:**

Replace `.unwrap()` with `.map_err(|e| anyhow!("Failed to serialize ...: {}", e))?` for consistency with the `NewEpochEvent` branch.

---

### GAPTOS-R2-004 — `DKGState::last_complete()` Panics on Missing State

**Severity:** MEDIUM  
**Status:** Open

**Description:**

In [dkg/mod.rs:429-431](file:///home/neko/gravity-aptos/types/src/dkg/mod.rs#L429-L431):

```rust
pub fn last_complete(&self) -> &DKGSessionState {
    self.last_completed.as_ref().unwrap()
}
```

This method panics if `last_completed` is `None`. The safe variant `maybe_last_complete()` exists but is gated on epoch matching. Any caller of `last_complete()` without prior validation will crash if DKG state hasn't been populated yet (e.g., during genesis or first epoch).

**Impact:**

- **Availability:** Panics during startup or epoch transitions if DKG state is not yet initialized.
- **Edge case:** Could occur during network bootstrap or after state recovery.

**Recommendation:**

Either (a) change `last_complete()` to return `Option<&DKGSessionState>` or `Result<&DKGSessionState>`, or (b) audit all callers to ensure they check `last_completed.is_some()` first.

---

### GAPTOS-R2-005 — `GravityExtension` Not Included in Transaction Crypto Hash

**Severity:** MEDIUM  
**Status:** Open

**Description:**

The `SignedTransaction` struct includes a `g_ext: GravityExtension` field ([mod.rs:615](file:///home/neko/gravity-aptos/types/src/transaction/mod.rs#L615)), and `PartialEq` correctly compares `g_ext` ([mod.rs:623](file:///home/neko/gravity-aptos/types/src/transaction/mod.rs#L623)). However, `SignedTransaction` does not derive `BCSCryptoHash` directly — the transaction hash is computed via `Transaction::UserTransaction(self.clone()).hash()` at [mod.rs:926](file:///home/neko/gravity-aptos/types/src/transaction/mod.rs#L926).

If `GravityExtension` is stripped or modified after signing but before hashing, the `block_id`, `txn_index_in_block`, and `txn_count_in_block` fields could be tampered with without invalidating the transaction signature.

The `RawTransaction` (which is what gets signed by `authenticator.verify(&self.raw_txn)`) does **not** include `g_ext`, meaning the signature does not cover the `GravityExtension` data.

**Impact:**

- **Integrity:** `GravityExtension` fields (`block_id`, `txn_index_in_block`, `txn_count_in_block`) are not covered by the transaction signature, making them malleable.
- **Provenance:** A relayer or intermediary could alter `g_ext` without detection.

**Recommendation:**

Verify whether `g_ext` is intended to be mutable/unsigned metadata (similar to `committed_hash` caching), or whether it should be included in the signing payload. If the latter, include `g_ext` in the `RawTransaction` or add a secondary signature covering it.

---

### GAPTOS-R2-006 — `new_v2_with_type_tag_str` Panics on Invalid Input

**Severity:** MEDIUM  
**Status:** Open

**Description:**

In [contract_event.rs:128-133](file:///home/neko/gravity-aptos/types/src/contract_event.rs#L128-L133):

```rust
pub fn new_v2_with_type_tag_str(type_tag_str: &str, event_data: Vec<u8>) -> Self {
    ContractEvent::V2(ContractEventV2::new(
        TypeTag::from_str(type_tag_str).unwrap(),
        event_data,
    ))
}
```

`TypeTag::from_str().unwrap()` will panic on malformed type tag strings. If this function is called with untrusted or externally-derived input (e.g., from event replays, state sync, or API deserialization), it crashes the node.

**Impact:**

- **Availability:** Panics if the type tag string is invalid.

**Recommendation:**

Change the return type to `Result<Self>` and propagate the error, or validate the input before calling.

---

### GAPTOS-R2-007 — `TransactionPayload::into_entry_function()` Panics

**Severity:** LOW  
**Status:** Open

**Description:**

In [mod.rs:537-542](file:///home/neko/gravity-aptos/types/src/transaction/mod.rs#L537-L542):

```rust
pub fn into_entry_function(self) -> EntryFunction {
    match self {
        Self::EntryFunction(f) => f,
        payload => panic!("Expected EntryFunction(_) payload, found: {:#?}", payload),
    }
}
```

This convenience method panics if the payload is not an `EntryFunction`. While this is a common Rust pattern for "known-good" paths, any misuse will crash the node.

**Recommendation:**

Add a `try_into_entry_function()` variant returning `Option<EntryFunction>` or `Result<EntryFunction>`, and audit callers to prefer the safe version.

---

### GAPTOS-R2-INFO-001 — Inconsistent Consensus Config TODO Comment

**Severity:** INFO  
**Status:** Open

**Description:**

In [consensus_config.rs:425](file:///home/neko/gravity-aptos/types/src/on_chain_config/consensus_config.rs#L425):

```rust
// TODO(gravity_alex): Some diff for aptos and gravity, need to check
```

This TODO comment indicates that the consensus configuration may have unresolved differences between Aptos and Gravity configurations that have not been fully reviewed or reconciled. While not a direct vulnerability, unreconciled configuration differences could lead to consensus incompatibilities or unexpected behavior.

**Recommendation:**

Resolve the TODO by documenting the intentional differences between Aptos and Gravity consensus configurations, or align them as appropriate.

---

## Cross-Reference with Round 1

The following round 1 findings are related to issues found in this round:

| Round 1 ID | Status | Related Round 2 Finding |
|-----------|--------|------------------------|
| GAPTOS-005 (`.expect()` usage) | Fixed (partially) | GAPTOS-R2-002 — `From<GravityEvent>` still uses `.expect()` |
| GAPTOS-008 (error handling) | Fixed | GAPTOS-R2-003 — `bcs::to_bytes().unwrap()` remains in 2 branches |

---

## Scope Notes

### Not in scope for this audit:
- Move smart contracts / framework modules
- Third-party dependencies (upstream Aptos code unchanged from fork)
- Network/P2P layer
- Cryptographic primitive implementations

### Methodology:
1. Pattern scanning: `todo!()`, `unimplemented!()`, `.unwrap()`, `.expect()`, `unsafe`, `panic!`
2. Gravity-specific code path analysis: `GTxnBytes`, `GravityExtension`, `GravityEvent`, `DKGSessionMetadata` conversion
3. Cross-referencing with round 1 findings for regressions or incomplete fixes
4. Manual review of critical production code paths (VM execution, API, consensus, event handling)
