### 📋 问题总览（按优先级排序）

| **ID** | **优先级** | **作者** | **问题描述** | **位置** | 状态 |
| --- | --- | --- | --- | --- | --- |
| P0-B4-1 | 🔴 P0 | alexyue | 核心配置路径连续 unwrap()，节点可 panic | event-notifications/src/[lib.rs](http://lib.rs) | Reviewer: alexyue, accept |
| P0-C3-1 | 🔴 P0 | alexyue | 未知 JWK 类型直接 panic!() | contract_[event.rs](http://event.rs) | Reviewer: alexyue, reject（不应出现的路径，需要 panic） |
| P0-D1-1 | 🔴 P0 | keanji-x | Cargo.lock 被加入 .gitignore | .gitignore | 符合预期 |
| P0-E3-1 | 🔴 P0 | keanji-x | VM 验证被完全禁用，任意数据可入 mempool | shared_mempool/[tasks.rs](http://tasks.rs) | 符合预期 |
| P1-B2-1 | 🟠 P1 | alexyue | consensus config 反序列化跳过内层 BCS | consensus_[config.rs](http://config.rs) | Reviewer: alexyue, accept（添加注释说明） |
| P1-B3-1 | 🟠 P1 | alexyue | todo!() 在 DKG event 生产路径 | contract_[event.rs](http://event.rs) | Reviewer: alexyue, accept（已在后续 commit 修复） |
| P1-B4-2 | 🟠 P1 | alexyue | OnChainConfigProvider from_str + unwrap | on_chain_config | Reviewer: alexyue, accept |
| P1-C1-1 | 🟠 P1 | alexyue | JWK 事件序列化混用 serde_json 与 bcs | contract_[event.rs](http://event.rs) | Reviewer: alexyue, accept（添加注释说明） |
| P1-A1-1 | 🟠 P1 | alexyue | MemProfiler 所有方法替换为 todo!() | memory_[profiler.rs](http://profiler.rs) | Reviewer: alexyue, reject（Aptos memprofiler 不兼容 Gravity） |
| P1-E3-2 | 🟠 P1 | keanji-x | async→sync 重大 API 变更未在 PR 标题反映 | CoreMempoolTrait | commit msg 问题，不需要修复 |
| P1-E4-1 | 🟠 P1 | keanji-x | VM validator pool 初始化被注释，pool 始终为空 | vm_[validator.rs](http://validator.rs) | https://github.com/Galxe/gravity-aptos/pull/49/changes |
| P1-F3-1 | 🟠 P1 | keanji-x | write_all 返回类型不匹配，日志轮转计算错误 | tracing_[writer.rs](http://writer.rs) | https://github.com/Galxe/gravity-aptos/pull/49/changes |
| P2-B1-1 | 🟡 P2 | alexyue | 31 文件大量代码被注释而非删除 | 多文件 | Reviewer: alexyue, reject（保留注释供参考） |
| P2-B1-2 | 🟡 P2 | alexyue | proptest 中 todo!() 残留 | proptest_[types.rs](http://types.rs) | Reviewer: alexyue, reject（当前必须保留） |
| P2-B2-2 | 🟡 P2 | alexyue | 大量 consensus 类型被注释保留 | consensus_[config.rs](http://config.rs) | Reviewer: alexyue, reject（保留注释供参考） |
| P2-B3-2 | 🟡 P2 | alexyue | Into trait 反向实现 + unwrap 无错误处理 | contract_[event.rs](http://event.rs) | Reviewer: alexyue, accept |
| P2-B4-3 | 🟡 P2 | alexyue | 注释掉的 DB 读取代码未清理 | on_chain_config | Reviewer: alexyue, reject（保留注释供参考） |
| P2-B8-1 | 🟡 P2 | alexyue | unwrap 修复不完全，仍有遗留 | event-notifications | Reviewer: alexyue, accept |
| P2-C1-2 | 🟡 P2 | alexyue | JWK 配置反序列化注释掉 MoveAny 解包 | jwk_consensus_[config.rs](http://config.rs) | Reviewer: alexyue, accept（添加注释说明） |
| P2-C2-1 | 🟡 P2 | alexyue | GLOBAL_RELAYER.get().unwrap() 可能 panic | jwk_observer | Reviewer: alexyue, accept |
| P2-D3-1 | 🟡 P2 | lightman | 非活跃 DKG config 变体填充全零默认值 | dkg converter |  |
| P2-E1-1 | 🟡 P2 | keanji-x | Trait 方法签名过长（7 参数） | CoreMempoolTrait | 无需修复 |
| P2-E2-1 | 🟡 P2 | keanji-x | add_txn 标记 async 但实现同步，有死锁风险 | CoreMempoolTrait | https://github.com/Galxe/gravity-aptos/pull/49/changes |
| P2-F2-1 | 🟡 P2 | keanji-x | 文件操作中使用 expect()，磁盘异常时 panic | SizeRollingFileAppender | https://github.com/Galxe/gravity-aptos/pull/49/changes |
| P3-B3-3 | 🟢 P3 | alexyue | epoch 字段改为 pub 破坏封装性 | NewEpochEvent | Reviewer: alexyue, reject（必须 pub 供外部访问） |
| P3-B5-1 | 🟢 P3 | alexyue | 文件末尾缺少换行符 | idl 模块 | Reviewer: alexyue, accept |
| P3-C1-3 | 🟢 P3 | alexyue | JwkIdlError 命名误导（JSON vs BCS） | jwk_converter | Reviewer: alexyue, reject |
| P3-D3-2 | 🟢 P3 | lightman | 驼峰命名违反 Rust 规范 | dkg types |  |
| P3-A1-2 | 🟢 P3 | alexyue | Cargo.toml 依赖改为 Galxe fork，需确认维护状态 | Cargo.toml | Reviewer: alexyue, reject（fork 由 Galxe 维护，无问题） |
| P3-A3-1 | �� P3 | keanji-x | x25519-dalek 版本变更需确认安全审计 | Cargo.toml | 无需修复 |
| P3-F1-1 | 🟢 P3 | keanji-x | 直接使用版本号而非 workspace 引用 | Cargo.toml | 无需修复 |

> **统计**: 🔴 P0 × 4 | 🟠 P1 × 8 | 🟡 P2 × 12 | 🟢 P3 × 7 — 共 **31** 项
> 

---

**审计范围**: `ebfb4b7` → `805ad52` (28 commits, PR #9 ~ #27) **审计时间**: 2026-03-04 **代码库**: `/home/kenji/galxe/gravity-aptos`

---

## **审计总览与风险矩阵**

| **严重级别** | **数量** | **描述** |
| --- | --- | --- |
| 🔴 P0 Critical | 3 | 可导致节点宕机或安全漏洞 |
| 🟠 P1 High | 5 | 功能缺失或可能导致运行时异常 |
| 🟡 P2 Medium | 6 | 代码质量问题，可能影响维护性 |
| 🟢 P3 Low | 4 | 风格/规范问题 |

---

## **Category B: API Types & On-Chain Config 核心重构**

### **B1. `a7ca55fc` — Temporarily move all the error produced by ra**

**Author**: alexyue  | **Files**: 31 | **Impact**: ⚠️ 极大

**变更目的**: 为适配 Gravity 架构，大量注释掉 Aptos 原有的测试代码、类型定义和 mock 执行层代码。

**审计发现**:

**CAUTION**

**[P2-B1-1] 大量代码被注释而非删除** — 31 个文件中大量使用 `//` 注释掉整段代码（例如 `dag_fault_tolerance.rs` 中约 200 行测试代码），而不是直接删除。这导致代码膨胀、阅读困难，且可能引入隐藏的编译问题。

**WARNING**

**[P2-B1-2] `todo!()` 残留** — `types/src/proptest_types.rs` 中 `TransactionPayload::GTxnBytes(bytes) => todo!()` 虽然修复了语法（移除了花括号），但 `todo!()` 仍存在，如果 proptest 生成该变体会导致 panic。

- **架构影响**: 低 — 主要是 dead code 清理
- **测试覆盖**: 无新增测试（大量测试被注释掉）
- **风险评估**: P2 Medium

---

### **B2. `2de8a5c5` — Use pub use to export the moved on-chain consensus config**

**Author**: alexyue  | **Files**: 13 | **Impact**: ⚠️ 大

**变更目的**: 将 `ConsensusConfig` 相关类型从 `api-types` crate 通过 `pub use` 导出到 `types` crate，使得 types crate 能直接引用 api-types 中的共识配置定义。

**审计发现**:

**CAUTION**

**[P1-B2-1] consensus config 反序列化跳过内层 BCS** — `types/src/on_chain_config/consensus_config.rs` 中 `deserialize_into_config` 修改为：

```
rust

// let raw_bytes: Vec<u8> = bcs::from_bytes(bytes)?;
// TODO(gravity_alex): Some diff for aptos and gravity, need to check
letraw_bytes=bytes;
```

跳过了一层 BCS 解包。如果 Gravity 和 Aptos 的序列化格式不一致，这可能导致反序列化失败或解析出错误的值。**需要确认这个修改是否正确匹配 L1 合约的编码方式。**

**WARNING**

**[P2-B2-2] 大量 consensus 类型被注释** — `ConsensusConfigV1`、`ProposerElectionType`、`LeaderReputationType`、`ProposerAndVoterConfig` 等多个结构体被完全注释掉(约 100 行)，但保留在文件中。应当直接删除或移至独立模块。

- **架构影响**: 中 — 改变了 consensus config 的导入路径和反序列化行为
- **测试覆盖**: 有现有测试但添加了 `use api_types::u256_define::AccountAddress` 适配
- **风险评估**: P1 High（反序列化变更）

---

### **B3. `703585ba` — move useless file and define gravity events**

**Author**: alexyue  | **Files**: 8 | **Impact**: ⚠️ 中

**变更目的**: 清理无用文件，定义 `GravityEvent` 枚举（`NewEpoch`, `JWK`, `DKG`），建立 Gravity 事件模型。

**审计发现**:

**WARNING**

**[P1-B3-1] `todo!()` 在 production code 路径** — `types/src/contract_event.rs` 中 `GravityEvent::JWK => todo!()` 和 `GravityEvent::DKG => todo!()`。这两个分支在生产环境触发时会直接 panic。虽然后续 commit 修复了 JWK，但 DKG 的 `todo!()` 一直延续到了 `#19`。

**WARNING**

**[P2-B3-2] `Into` trait 反向实现** —

```
rust

implInto<ContractEvent>forGravityEvent {
fninto(self)->ContractEvent {
ContractEvent::try_from(self).unwrap()
    }
}
```

1. Rust 惯例应实现 `From` 而非 `Into`（clippy 会警告）
2. 使用 `.unwrap()` 无错误处理

**NOTE**

**[P3-B3-3]** `NewEpochEvent.epoch` 字段从 private 改为 `pub`，破坏了封装性但确实需要外部访问。

- **架构影响**: 中 — 引入了 Gravity 事件系统核心抽象
- **风险评估**: P1 High（`todo!()` in production）

---

### **B4. `5b07e60a` — refactor dbbackendprovider to use gravity config storage**

**Author**: alexyue  | **Files**: 13 | **Impact**: 🔴 极大

**变更目的**: 将 Aptos 原有的 DB-backed 配置提供者替换为 Gravity 全局配置存储（`GLOBAL_CONFIG_STORAGE`），使节点从 L1 合约读取配置而不是本地 StateDB。

**审计发现**:

**CAUTION**

**[P0-B4-1] 多处 `.unwrap()` 在核心配置路径** — `event-notifications/src/lib.rs` 中：

```
rust

letgravity_config_storage=GLOBAL_CONFIG_STORAGE.get();
letepoch_bytes=gravity_config_storage.unwrap()
.fetch_config_bytes(...)
.ok_or_else(||anyhow!("...")).unwrap();
letepoch=TryInto::<u64>::try_into(epoch_bytes).unwrap();
```

三连 `unwrap()` 在 epoch 获取路径上，任何一个失败都会导致节点 panic。**在共识关键路径上使用 unwrap 是不可接受的。**（注：后续 commit `4315691e` 修复了部分但不完全。）

**WARNING**

**[P1-B4-2] `OnChainConfigProvider` 使用 `from_str` + `unwrap`** —

```
rust

api_types::config_storage::OnChainConfig::from_str(T::TYPE_IDENTIFIER).unwrap()
```

如果 `TYPE_IDENTIFIER` 不在 `OnChainConfig` 枚举定义中，节点会 panic。

**NOTE**

**[P2-B4-3] 注释掉的 DB 读取代码** — 旧的 DB 读取逻辑被注释保留（约 20 行），增加代码噪声。

- **架构影响**: 极大 — 完全替换了 on-chain config 的获取机制
- **测试覆盖**: 无新测试
- **风险评估**: P0 Critical

---

### **B5. `0af96469` (PR #9) — Add ValidatorSet type for execution layer**

**Author**: alexyue  | **Files**: 9 | **Impact**: ⚠️ 中

**变更目的**: 添加 `ValidatorSet` IDL 类型及 `types/src/idl/` 模块，实现 `ValidatorInfo` 在 API types 和 types crate 之间的双向转换。

**审计发现**:

**TIP**

**[POSITIVE]** 这是本批修改中**代码质量最好**的 PR：

- 完整的错误类型定义（`ValidatorInfoIdlError`）
- 所有转换使用 `TryFrom` 而非 `unwrap()`
- 有 roundtrip 测试和格式验证测试
- 有 error handling 测试

**NOTE**

**[P3-B5-1]** 文件末尾缺少换行符 (`\ No newline at end of file`)

- **风险评估**: P3 Low — 高质量实现

---

### **B6. `8eff5075` — add epoch in ExternalBlockMeta**

**Author**: alexyue  | **Files**: 1 | **变更**: 1 行

添加 `pub epoch: u64` 到 `ExternalBlockMeta` 结构体。**简洁、正确。**

- **风险评估**: P3 Low

---

### **B7. `a9ae5df7` — pass proposer inside ExternalBlockMeta**

**Author**: alexyue  | **Files**: 1 | **变更**: 1 行

添加 `pub proposer: Option<ExternalAccountAddress>` 到 `ExternalBlockMeta`。使用 `Option` 保持向后兼容。**简洁、正确。**

- **风险评估**: P3 Low

---

### **B8. `4315691e` — don't panic in OnChainConfigProvider**

**Author**: ByteYue | **Files**: 1

**变更目的**: 修复 B4 引入的 `unwrap()` panic 问题，改为返回 Error。

**审计发现**:

**IMPORTANT**

**[P2-B8-1] 修复不完全** — 虽然修复了 `GLOBAL_CONFIG_STORAGE.get().unwrap()` 和 `fetch_config_bytes().unwrap()`，但仍遗留：

```
rust

letepoch=TryInto::<u64>::try_into(epoch_bytes).unwrap();// 仍然 unwrap!
```

以及 `OnChainConfigProvider::get` 中 的：

```
rust

gravity_config_storage.ok_or_else(||Error::UnexpectedErrorEncountered(...))?
// ... 但后面的 from_str().unwrap() 未修复
```

- **风险评估**: P2 Medium（部分修复）

## **Category C: JWK (JSON Web Key) 支持**

### **C1. `12e33272` (PR #11) — Support jwk in gravity**

**Author**: alexyue | **Files**: 18 | **Impact**: ⚠️ 大

**变更目的**: 实现 Gravity 的 JWK 支持：定义 `api-types` 中的 JWK 结构、JWK 转换器（`jwk_converter.rs`）、以及 Relayer trait。

**审计发现**:

**WARNING**

**[P1-C1-1] JWK 事件序列化使用 `serde_json` 而非 `bcs`** — `contract_event.rs` 中创建 NewEpoch 事件使用了 `serde_json::to_vec` ,但 JWK 事件使用 `bcs::to_bytes`。同一代码路径中混用两种序列化格式，容易产生反序列化错误。

```
rust

GravityEvent::NewEpoch(epoch,_)=> {
serde_json::to_vec(&data).unwrap()// NewEpoch 用 JSON
}
GravityEvent::ObservedJWKsUpdated(..)=> {
bcs::to_bytes(&data).unwrap()// JWK 用 BCS
}
```

**WARNING**

**[P2-C1-2] JWK 配置反序列化注释掉了 MoveAny 解包** — `jwk_consensus_config.rs` 中注释掉了原有的 `bcs::from_bytes::<MoveAny>` 逻辑，改用 `api_types::JWKConsensusConfig`。这需要 Gravity L1 合约返回的数据格式与此匹配。

**NOTE**

**[P3-C1-3]** `JwkIdlError::JsonDeserializationError` 命名误导 — 实际用于 BCS 反序列化错误，而非 JSON。

- **架构影响**: 大 — 引入完整的 JWK 桥接层
- **测试覆盖**: 无新测试（但有 JWK converter 的 roundtrip 逻辑）
- **风险评估**: P1 High

---

### **C2. `a224b0c7` (PR #12) — add UnsupportedJWK type in api onchain config types**

**Author**: alexyue | **Files**: 7

**变更目的**: 添加 `UnsupportedJWK` 类型，将 Relayer trait 的返回值从 `Vec<u8>` 改为 `Vec<JWKStruct>`，并为 `OIDCProvider` 添加 `onchain_block_number` 字段。

**审计发现**:

**NOTE**

**[P3-C2-1]** `fetch_jwks_with_relayer` 中 `GLOBAL_RELAYER.get().unwrap()` — Relayer 可能未初始化时会 panic。

- **风险评估**: P2 Medium

---

### **C3. `968523933` (PR #19) — Wrap PollResult by Relayer**

**Author**: alexyue | **Files**: 3

**变更目的**: 引入 `PollResult` 结构，为 JWK 事件转换添加 RSA_JWK / UnsupportedJWK 类型匹配。

**审计发现**:

**CAUTION**

**[P0-C3-1] `panic!()` 在事件处理路径** —

```
rust

_=>panic!("unknown jwk type: {}",jwk.type_name),
```

如果收到未知的 JWK 类型，节点直接 panic。**应返回 Error 而非 panic。**

- **风险评估**: P0 Critical

---

### **C4. `ad0908f4` (PR #16) — enhance (jwk_observer)**

**Author**: alexyue | **Files**: 1

**变更目的**: 格式化代码和改进错误处理 — 将 `unwrap()` 替换为 `map_err` + `?`。

**审计发现**: ✅ 正面变更，改善了 `fetch_jwks_with_relayer` 的错误处理。

- **风险评估**: P3 Low

---

## **Category D: DKG 支持**

### **D1. `11bb0e92` (PR #14) — add dkg lib**

**Author**: keanji-x | **Files**: 3

**变更目的**: 导出 `aptos-dkg` 和 `aptos-dkg-runtime` crate。

**审计发现**:

**CAUTION**

**[P0-D1-1] `Cargo.lock` 加入 `.gitignore`** —

```

+Cargo.lock
```

**对于应用程序（非 library），`Cargo.lock` 必须提交到版本控制。** 忽略 `Cargo.lock` 会导致不同开发者/CI 使用不同的依赖版本，可能引入不可复现的 bug。

- **风险评估**: P0 Critical

---

### **D2. `ef69a6d2` (PR #15) — add lib (cleanup)**

**Author**: keanji-x | **Files**: 1

移除了 PR #14 中误加的 `pub use aptos_dkg;`（重复导出）。**修复性提交，正确。**

- **风险评估**: P3 Low

---

### **D3. `c13e7763` (PR #23) — Support to dkg struct in api-type**

**Author**: lightman | **Files**: 11 | **Impact**: ⚠️ 大

**变更目的**: 为 DKG 添加完整的 API types 定义和 types 层面的 converter，包括 `DKGSessionMetadata`、`DKGSessionState`、`DKGState`，以及 `RandomnessConfig` 的序列化适配。

**审计发现**:

**WARNING**

**[P2-D3-1] 非活跃变体使用全零默认值** — 在 `to_api_types` 中，非当前变体的 config 被填充为全零：

```
rust

letconfig_v2=ConfigV2 {
    secrecyThreshold:FixedPoint64 {value:0 },
    reconstructionThreshold:FixedPoint64 {value:0 },
    fastPathSecrecyThreshold:FixedPoint64 {value:0 },
};
```

这可能导致下游代码误认为该 config 有效。建议使用 `Option` 包装。

**NOTE**

**[P3-D3-2]** 使用驼峰命名（`secrecyThreshold`, `configV1`）违反 Rust 命名规范。但如果这是为了匹配 Solidity ABI，可以接受。

**TIP**

**[POSITIVE]** DKG converter 提供了完整的双向转换（`from_api_types` / `to_api_types`），错误处理使用 `Result`，代码结构清晰。

- **测试覆盖**: 无新测试
- **风险评估**: P2 Medium

---

## **Category E: Mempool 实现**

### **E1. `215f0166` (PR #17) — add shared_mempool and export it**

**Author**: keanji-x | **Files**: 16 | **Impact**: 🔴 极大

**变更目的**: 引入 `CoreMempoolTrait` 抽象 trait 和 `GravityCoreMempool` wrapper，使 mempool 可以被 Gravity 外部实现替换。将所有对 `CoreMempool` 的直接引用替换为 `Box<dyn CoreMempoolTrait>`。

**审计发现**:

**IMPORTANT**

**[P2-E1-1] Trait 方法签名过长** — `add_txn` 接受 7 个参数且每行超长，建议引入参数结构体：

```
rust

fnadd_txn(&mutself,txn:SignedTransaction,ranking_score:u64,
sequence_info:u64,timeline_state:TimelineState,
client_submitted:bool,ready_time_at_sender:Option<u64>,
priority:Option<BroadcastPeerPriority>)->MempoolStatus;
```

**NOTE**

**[POSITIVE]** 抽象设计合理 — `CoreMempoolTrait` 保留了所有必要的方法，`GravityCoreMempool` 是简单的委托实现，不会引入行为变更。

- **架构影响**: 大 — 改变了 mempool 的所有权模型
- **测试覆盖**: 测试代码同步更新
- **风险评估**: P2 Medium

---

### **E2. `6649dd9f` (PR #18) — implement core mempool**

**Author**: keanji-x | **Files**: 7

**变更目的**: 给 `CoreMempoolTrait` 添加 `#[async_trait]`，使 `add_txn` 和 `reject_transaction` 变为 async method。

**审计发现**:

**WARNING**

**[P2-E2-1] `add_txn` 设为 async 但实现同步** — `GravityCoreMempool::add_txn` 标记为 `async` 但实际内部调用是同步的 `self.0.add_txn(...)`。这增加了不必要的 async 开销（Future state machine 生成），且 `Mutex<Box<dyn CoreMempoolTrait>>` 在 async context 中持锁调用 `.await` 可能导致 **死锁风险**。

**NOTE**

PR #21 后续又将 async 改回了 sync，说明这个 async 设计有问题。这 2 个 commit 之间不必要的来回变更浪费了代码审查时间。

- **风险评估**: P2 Medium

---

### **E3. `8175664f` (PR #21) — fix some bugs in broadcast**

**Author**: keanji-x | **Files**: 8 | **Impact**: 🔴 极大

**变更目的**: 修复 broadcast 相关 bug，但实际做了一个极大的改动：**完全禁用了 VM validation**。

**审计发现**:

**CAUTION**

**[P0-CRITICAL: E3-1] VM 验证被完全禁用** — `mempool/src/shared_mempool/tasks.rs` 中：

- 整个 `VALIDATION_POOL.install()` 并行验证块被注释掉（约 15 行）
- 所有 validation_result 匹配逻辑被注释掉（约 25 行）
- `ranking_score` 硬编码为 `0`
- 事务直接进入 mempool，**无任何格式验证、签名验证、gas 检查**

这意味着**任何人都可以向 mempool 提交任意数据**，包括：

- 签名无效的交易
- gas 不足的交易
- 格式错误的交易

**如果这是有意的（Gravity 使用不同的验证机制），需要有明确的文档说明和替代验证逻辑。**

**WARNING**

**[P1-E3-2] async → sync 回退** — `CoreMempoolTrait` 从 async 回退为 sync，但 PR 标题("fix some bugs in broadcast") 完全没有反映这个重大 API 变更。

- **风险评估**: P0 Critical（VM validation 禁用需要确认是否有意为之）

---

### **E4. `29113c0c` (PR #20) — remove vm validator**

**Author**: keanji-x | **Files**: 1

**变更目的**: 禁用 `PooledVMValidator` 的初始化。

**审计发现**:

**WARNING**

**[P1-E4-1] VM Validator pool 初始化被注释** —

```
rust

// for _ in 0..pool_size {
//     vm_validators.push(Arc::new(Mutex::new(VMValidator::new(db_reader.clone()))));
// }
```

导致 `PooledVMValidator::vm_validators` 始终为空 Vec。任何尝试使用 validator pool 的代码可能 index out of bounds panic。**与 E3 的 VM validation 禁用逻辑配合使用。**

- **风险评估**: P1 High

---

## **Category A: 基础设施 & 构建工具**

### **A1. `e4b9ed42` — Use tikv-jemalloc**

**Author**: ByteYue | **Files**: 40

**变更目的**: 将 `jemallocator` / `jemalloc-sys` 替换为 `tikv-jemallocator` / `tikv-jemalloc-sys`，以兼容 Gravity (greth) 构建栈。修改了所有依赖 jemalloc 的 binary crate。

**审计发现**:

**WARNING**

**[P1-A1-1] MemProfiler 全部替换为 `todo!()`** — `crates/aptos-profiler/src/memory_profiler.rs` 中 `profile_for`、`start_profiling`、`end_profiling` 三个方法全部被替换为 `todo!()`。**内存 profiling 功能完全不可用**。如果任何代码路径触发 profiling，节点会 panic。

**NOTE**

**[P3-A1-2]** Cargo.toml 中 `processor` 和 `server-framework` 依赖 git 仓库从 `aptos-labs` 改为 `Galxe`。需要确认这些 fork 的维护状态。

- **风险评估**: P1 High（profiler `todo!()`）

---

### **A2. `36d301e2` (PR #10) — Update shadow-rs and git2**

**Author**: alexyue | **Files**: 5

**变更目的**: 更新 `shadow-rs` 和 `git2` 依赖版本以兼容 greth 构建，同时更新 rust-toolchain 版本。

- **审计发现**: ✅ 纯依赖版本更新，无逻辑变更。
- **风险评估**: P3 Low

---

### **A3. `6e6c5552` (PR #13) — fix: use new version x25519-dalek**

**Author**: keanji-x | **Files**: 2

**变更目的**: 更新 `x25519-dalek` crate 并修改 `PublicKey` 构造方式。

**审计发现**:

**NOTE**

**[P3-A3-1]** 密码学库版本变更需要特别注意。`x25519` 密钥交换算法对版本敏感，建议确认新版本的安全审计状态。

- **风险评估**: P3 Low

---

### **A4. `f3ab5a49` (PR #27) — refactor(api-types): use ExtraDataType enum**

**Author**: lightman | **Files**: 4

**变更目的**: 引入 `ExtraDataType` 枚举来区分 JWK 和 DKG 的额外数据类型，统一数据处理方式。

- **审计发现**: ✅ 清晰的重构，改进类型系统。
- **风险评估**: P3 Low

---

## **Category F: 日志 & 可观测性**

### **F1. `de72c593` (PR #25) — use tracing log**

**Author**: keanji-x | **Files**: 5

**变更目的**: 引入 `tracing-appender` 日志组件，添加 `TracingWriter` 实现非阻塞文件日志写入。

**审计发现**:

**NOTE**

**[P3-F1-1]** `Cargo.toml` 中直接使用版本号而非 workspace 引用：

```
toml

tokio = {version ="1.21.0",features = ["full"] }
tracing ="0.1.37"
```

应使用 `{ workspace = true }` 保持版本一致性。

- **风险评估**: P3 Low

---

### **F2. `3acef9f5` (PR #26) — add rotation log**

**Author**: keanji-x | **Files**: 5

**变更目的**: 实现基于文件大小的日志轮转（`SizeRollingFileAppender`），可配置最大文件大小和最大文件数量。同时升级 `tracing-subscriber` 到 0.3.20。

**审计发现**:

**TIP**

**[POSITIVE]** 自定义日志轮转实现合理：

- 支持 `max_log_file_size_mbs` 和 `max_log_files` 配置
- 默认值合理：200MB/文件，保留 10 个
- 轮转逻辑正确（倒序重命名）

**WARNING**

**[P2-F2-1] `expect()` 在文件操作中** — `open_log_file`、`rotate` 中多处使用 `expect("Failed to ...")`。在生产环境中，磁盘满或权限错误时会导致节点 panic。

- **风险评估**: P2 Medium

---

### **F3. `805ad528` — add new line tag in log**

**Author**: keanji-x | **Files**: 1

**变更目的**: 在日志写入后添加换行符。

**审计发现**:

**CAUTION**

**[P1-F3-1] 类型不匹配导致编译错误或逻辑 bug** —

```
rust

let mut bytes_written=self.current_log_file.write(buf)?;
bytes_written+=self.current_log_file.write_all(b"\n")?;
```

`write_all` 返回 `io::Result<()>` 而非 `io::Result<usize>`。将 `()` 加到 `usize` 上：

- 如果编译通过（通过 trait 隐式转换），`bytes_written` 不会增加换行符的字节数
- 导致 `current_log_size` 追踪不准确，可能延迟日志轮转

正确写法：

```
rust

letbytes_written=self.current_log_file.write(buf)?;
self.current_log_file.write_all(b"\n")?;
// 总计 bytes_written + 1
```

- **风险评估**: P1 High

---

### **F4. `1b9a3437` (PR #22) — remove verbose "verify with public key" log**

**Author**: nekomoto911 | **Files**: 1

**变更目的**: 移除 `5b07e60a` 中引入的 `tracing::info!("author {} verify with public key")` 日志（每次签名验证都打印，极为冗余），保留 error 日志。

- **审计发现**: ✅ 正确清理。
- **风险评估**: P3 Low

---

### **F5. `9899ff92` (PR #24) — remove some log**

**Author**: keanji-x | **Files**: 3

**变更目的**: 移除 mempool 中的 `info!("add txn to mempool")` 日志（PR #21 引入），将 `use_case_history` 中的 info 降级为 debug，将 `on_disk.rs` 中的 info 降级为 debug。

- **审计发现**: ✅ 合理的日志级别调整。
- **风险评估**: P3 Low

---

## **关键发现汇总**

### **🔴 P0 Critical 问题（3个）**

| **ID** | **位置** | **问题** | **建议** |
| --- | --- | --- | --- |
| P0-B4-1 | `event-notifications/src/lib.rs` | 核心配置路径连续 unwrap() | 全部改为 `?` 错误传播 |
| P0-C3-1 | `contract_event.rs` | 未知 JWK 类型直接 `panic!()` | 改为 log + return Error |
| P0-D1-1 | `.gitignore` | `Cargo.lock` 被 gitignore | 立即从 .gitignore 移除 |

### **🟠 P1 High 问题（5个）**

| **ID** | **位置** | **问题** | **建议** |
| --- | --- | --- | --- |
| P1-B2-1 | consensus_config.rs | 跳过内层 BCS 反序列化 | 确认并文档化格式差异 |
| P1-B3-1 | contract_event.rs | `todo!()` 在 DKG event 路径 | 实现或返回 Error |
| P1-A1-1 | memory_profiler.rs | 所有 profiling 方法为 `todo!()` | 适配 tikv-jemalloc 或移除 |
| P1-E4-1 | vm_validator.rs | VM validator pool 为空 | 确认是否有意为之并文档化 |
| P1-F3-1 | tracing_writer.rs | `write_all` 返回类型不匹配 | 修复返回值计算 |

### **特别关注：E3 VM Validation 禁用**

PR #21 (`8175664f`) 完全禁用了 mempool 的 VM 验证。结合 PR #20 的 VM Validator pool 清空，**所有交易在进入 mempool 时不经过任何验证**。

**CAUTION**

**如果这是设计意图（Gravity 使用 L1 侧的验证机制），必须：**

1. 在代码中添加明确注释说明原因
2. 确认有替代的验证机制
3. 删除注释掉的代码而非保留
4. 更新 PR 描述反映这个重大变更

---

## **整体评价**

### **代码质量**

- **注释掉代码**: 多处使用 `//` 注释大段代码，而非删除。建议统一清理。
- **命名规范**: 部分 Rust 代码使用驼峰命名（如 DKG 类型），不符合 Rust 约定。
- **错误处理**: `unwrap()` / `panic!()` / `todo!()` 在生产路径上使用过多。

### **架构**

- **全局状态**: 大量使用 `OnceLock` (`GLOBAL_CONFIG_STORAGE`, `GLOBAL_RELAYER`, `GLOBAL_WRITER`)。虽然在节点程序中可以接受，但增加了测试难度。
- **IDL 层设计**: `types/src/idl/` 模块提供了清晰的跨层类型转换，是好的架构决策。
- **Mempool 抽象**: `CoreMempoolTrait` 的抽象设计合理，使 Gravity 可以替换 mempool 实现。

### **测试**

- 大量原有测试被注释掉，新增功能的测试覆盖不足
- 唯一有良好测试的是 PR #9 的 ValidatorInfo IDL
