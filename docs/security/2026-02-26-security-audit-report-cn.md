# 安全审计报告 — gravity-aptos

**日期：** 2026-02-26
**范围：** Gravity 特定变更（约54个 commit），涉及共识、DKG、网络、内存池、JWK、API、中继器、IDL、配置和加密模块
**语言：** Rust
**仓库：** `Galxe/gravity-aptos`（分支：`aptos-node`，commit `c9ae04b`）

---

## 摘要

| 严重程度 | 数量 | 状态 |
|----------|------|------|
| 严重 | 1 | **已修复**（2026-02-27，分支 `security-audit-fixes`）|
| 高 | 5 | **已修复** |
| 中 | 8 | 5 已修复，3 延后 |
| 低 | 9 | 4 已修复，5 延后 |
| 信息 | 8 | 3 已修复，5 延后/无需处理 |
| **总计** | **31** | **18 已修复，13 延后** |

---

## 严重（1）

### GAPTOS-001：NewEpochEvent 序列化不匹配（serde_json vs BCS）

**文件：** `types/src/contract_event.rs:478-483`

**问题：** 将 `GravityEvent::NewEpoch` 转换为 `ContractEvent` 时，`NewEpochEvent` 数据使用 `serde_json::to_vec()` 序列化：

```rust
GravityEvent::NewEpoch(data) => ContractEvent::new_v2_with_type_tag_str(
    "0x1::reconfiguration::NewEpochEvent",
    serde_json::to_vec(&data).unwrap(),  // <-- JSON
)
```

但反序列化路径 `NewEpochEvent::try_from_bytes()`（`types/src/account_config/events/new_epoch.rs:31`）使用的是 `bcs::from_bytes()`。其他两个事件类型（`ObservedJWKsUpdated` 第527行和 `DKGStartEvent` 第539行）正确使用了 `bcs::to_bytes()`。

**影响：** 来自 `GravityEvent::NewEpoch` 的纪元转换事件将无法反序列化，可能导致纪元转换无法被识别。如果在共识过程中触发此代码路径，可能导致链停止。

**建议：** 将第482行的 `serde_json::to_vec(&data).unwrap()` 改为 `bcs::to_bytes(&data).unwrap()`。

**审查意见** 审查人: Lightman; 状态: 已接受; 备注:

---

## 高（5）

### GAPTOS-002：Noise 握手中移除了 Peer ID 验证

**文件：** `network/framework/src/noise/handshake.rs:390-411`

**问题：** Commit `977f5b93` 移除了在 Noise IK 握手期间验证非信任 peer 的 `peer_id` 是否从其加密公钥派生的检查。原始 Aptos 代码验证：

```rust
// 被 Gravity 移除：
let derived_remote_peer_id = from_identity_public_key(remote_public_key);
if derived_remote_peer_id != remote_peer_id {
    return Err(NoiseHandshakeError::ClientPeerIdMismatch(...));
}
```

在 VFN 和公共网络（使用 `MaybeMutual` 认证）上，任何 peer 现在可以使用任何有效的 x25519 密钥对声明任意 `peer_id`。

**影响：** 非验证器网络上的 peer 身份伪造。攻击者可以冒充任何 peer ID，绕过连接限制（因为限制是按 peer ID 计算的），并干扰合法的 peer 连接。验证器网络（`Mutual` 认证）不受影响。

**建议：** 恢复对非信任 peer 的 peer ID 派生检查，或添加一个明确的配置标志来禁用它，并提供清晰的安全影响说明。

**审查意见** 审查人: Lightman; 状态: 已拒绝; 备注: Gravity VFN 的 peer ID 使用 Aptos 账户地址而非网络公钥，因此原始的 peer ID 派生检查不适用

---

### GAPTOS-003：硬编码 DKG 随机性配置绕过链上配置

**文件：** `dkg/src/epoch_manager.rs:170-174`

**问题：** 链上随机性配置序列号被硬编码为 `0` 并带有 TODO 注释：

```rust
// TODO(gravity_lightman_dkg): mock randomness config seq num
let onchain_randomness_config_seq_num = RandomnessConfigSeqNum { seq_num: 0 };
```

读取链上值的原始代码被注释掉了。`local_seqnum > onchain_seqnum` 检查（第184行）始终与 0 比较。

**影响：** 通过链上配置强制禁用随机性的安全机制已失效。在需要全链禁用随机性的紧急情况下，验证器将忽略链上序列号。

**建议：** 恢复链上配置读取，或添加 feature flag 而非硬编码。

**审查意见** 审查人: Lightman; 状态: 已接受; 备注:

---

### GAPTOS-004：验证器集反序列化 `unwrap()` 在格式错误输入时 panic

**文件：** `types/src/idl/api_types_converter.rs:27-30`

**问题：** `construct_and_convert_validator_set()` 在 BCS 反序列化上调用 `.unwrap()`，尽管函数返回 `Result`：

```rust
let validator_set = bcs::from_bytes::<...>(bytes)
    .map_err(|e| format_err!("..."))
    .unwrap();  // <-- PANIC 而非 Err
```

此函数在纪元转换期间从 `ValidatorSet::deserialize_into_config()` 调用。

**影响：** 来自 greth 的格式错误验证器集数据将导致共识节点崩溃，而非返回优雅的错误。如果攻击者能影响执行层数据，可被利用来停止共识。

**建议：** 将 `.unwrap()` 替换为 `?` 以实现正确的错误传播。

**审查意见** 审查人: Lightman; 状态: 已接受; 备注:

---

### GAPTOS-005：DKG 验证器集转换中的 `unwrap()`

**文件：** `types/src/dkg/mod.rs:92-106`

**问题：** `target_validator_consensus_infos_cloned()` 和 `dealer_consensus_infos_cloned()` 都在单个验证器信息转换上调用 `.unwrap()`：

```rust
self.target_validator_set.clone().into_iter()
    .map(|obj| obj.try_into().unwrap())  // <-- 无效 BLS 密钥时 PANIC
    .collect()
```

`try_into()` 解析 BLS12381 公钥字节，对格式错误的密钥可能失败。

**影响：** 单个拥有损坏共识公钥的验证器将导致所有其他验证器的 DKG 过程崩溃，阻止纪元转换。

**建议：** 返回 `Result<Vec<ValidatorConsensusInfo>>` 或过滤无效条目并记录日志。

**审查意见** 审查人: Lightman; 状态: 已接受; 备注:

---

### GAPTOS-006：事件转换中未知 JWK 类型导致 `panic!()`

**文件：** `types/src/contract_event.rs:517`

**问题：** 在 `TryFrom<&GravityEvent> for ContractEvent` 中，未知的 JWK 类型会导致 `panic!()`：

```rust
_ => panic!("unknown jwk type: {}", jwk.type_name),
```

此外，`impl Into<ContractEvent> for GravityEvent`（第548行）用 `.unwrap()` 包装了 `TryFrom`。

**影响：** 来自中继器或源链的意外 JWK 类型将导致所有处理该事件的验证器崩溃。这是一个 DoS 攻击向量。

**建议：** 将 `panic!()` 替换为 `Err(anyhow!(...))`。移除执行 unwrap 的 `Into` 实现。

**审查意见** 审查人: Lightman; 状态: 已接受; 备注:

---

## 中（8）

### GAPTOS-007：共识 `RoundProposer` 选举类型中的 `todo!()`

**文件：** `consensus/src/epoch_manager.rs:390`

**问题：** 如果链上配置设置了 `RoundProposer` 选举类型，`todo!("not invoked in Gravity")` 将导致崩溃。

**影响：** 设置此选举类型的治理提案将导致所有验证器崩溃。

**建议：** 替换为错误返回或降级处理。

**审查意见** 审查人: Lightman; 状态: 已拒绝; 备注: 这是 Aptos 共识代码，已复制到 gravity-sdk，gravity-aptos 中不使用

---

### GAPTOS-008：LedgerInfo 扩展的 `block_hash`/`block_number` 改变了 BCS 哈希

**文件：** `types/src/ledger_info.rs:52-62`

**问题：** 两个新字段（`block_hash: HashValue`、`block_number: u64`）被包含在 `BCSCryptoHash` 派生中。如果验证器计算出不同的值，它们的 LedgerInfo 哈希将不同，阻止法定人数证书的形成。

**影响：** 如果验证器在 `block_hash` 或 `block_number` 上存在分歧，共识将停止。

**建议：** 确保这些字段在签名前被确定性地设置。记录它们何时/如何被设置。

**审查意见** 审查人: Lightman; 状态: 延后; 备注: 修复需要更新代码库中 92 个 `LedgerInfo::new()` 调用者；保持 gravity-sdk 中 `set_block_hash`/`set_block_number` setter 的现有方式

---

### GAPTOS-009：JWK Manager 不回收已撤销的提供者

**文件：** `crates/aptos-jwk-consensus/src/jwk_manager/mod.rs:267-374`

**问题：** 增量 JWK 更新模型从不移除已从链上配置中撤销的 OIDC 提供者。过期的 JWK 状态无限期持续。

**影响：** 已撤销提供者的 JWK 仍然有效，可能允许使用已弃用的凭据进行认证。

**建议：** 添加提供者清理逻辑，在提供者从链上配置中移除时删除 JWK 条目。

**审查意见** 审查人: Lightman; 状态: 待定; 备注: @AlexYue

---

### GAPTOS-010：JWK 排序对所有来源禁用（不仅限于 Gravity）

**文件：** `crates/aptos-jwk-consensus/src/jwk_observer.rs:95-97`

**问题：** JWK 排序被注释掉，注释说"返回的 jwk 已经排序"：

```rust
// In gravity oracle, we shouldn't do sort since the returned jwks are already sorted.
// jwks.sort();
```

这适用于所有 JWK 来源，不仅限于 `gravity://` 前缀的来源。HTTPS OIDC 提供者可能返回未排序的 JWK。

**影响：** HTTPS 提供者的非确定性 JWK 排序可能导致共识分歧。

**建议：** 仅对 `gravity://` 来源跳过排序。添加验证排序顺序的调试断言。

**审查意见** 审查人: Lightman; 状态: 待定; 备注: @AlexYue

---

### GAPTOS-011：X25519 密钥生成忽略提供的 RNG

**文件：** `crates/aptos-crypto/src/x25519.rs:177-184`

**问题：** `x25519::PrivateKey` 的 `Uniform::generate()` 忽略 `rng` 参数，始终使用 `OsRng`：

```rust
fn generate<R>(_rng: &mut R) -> Self
where R: rand_core::CryptoRng + rand_core::RngCore,
{
    Self(x25519_dalek::StaticSecret::random())
}
```

**影响：** 期望确定性密钥生成的测试将获得非确定性结果。违反了 `Uniform` trait 契约。

**建议：** 使用 `x25519_dalek::StaticSecret::random_from_rng(rng)`。

**审查意见** 审查人: Lightman; 状态: 已拒绝; 备注: 原始 Aptos 代码，需要验证对 Gravity 的适用性

---

### GAPTOS-012：磁盘存储在 Debug 级别记录所有键值数据

**文件：** `secure/storage/src/on_disk.rs:64`

**问题：** `debug!("OnDiskStorage::read: path {:?}, data {:?}", self.file_path, data)` 在启用调试日志时记录包括加密密钥在内的全部存储内容。

**影响：** 私钥材料可能被写入日志文件。

**建议：** 从日志中编辑数据或仅记录密钥名称。

**审查意见** 审查人: Lightman; 状态: 已拒绝; 备注: Aptos DB 相关代码，Gravity 中不使用

---

### GAPTOS-013：网络地址解析静默降级为空

**文件：** `types/src/idl/api_types_converter.rs:103-122`

**问题：** `parse_network_address()` 在两种反序列化策略都失败时静默返回 `Ok(vec![])`。没有日志或错误。

**影响：** 网络地址格式错误的验证器将静默变得不可达。

**建议：** 返回错误或记录警告。

**审查意见** 审查人: Lightman; 状态: 已接受; 备注: 在两种反序列化策略都失败时添加了 `tracing::warn!`

---

### GAPTOS-014：OnChainConsensusConfig 反序列化更改带有 TODO

**文件：** `types/src/on_chain_config/consensus_config.rs:423-429`

**问题：** 原始 Aptos 的双重 BCS 反序列化被改为单轮，带有 TODO：

```rust
// TODO(gravity_alex): Some diff for aptos and gravity, need to check
let raw_bytes = bytes;
```

对应的测试仍然使用双重序列化。

**影响：** 执行层和共识层之间的格式不匹配将导致配置反序列化失败。

**建议：** 验证格式一致性。修复或移除过时的测试。移除 TODO。

**审查意见** 审查人: Lightman; 状态: 待定; 备注: @AlexYue

---

## 低（9）

### GAPTOS-015：ChainId `FromStr` 仍按 `u8` 解析

**文件：** `types/src/chain_id.rs:186`
**问题：** `ChainId` 已从 `u8` 扩展为 `u64`，但 `FromStr` 仍按 `u8` 解析，拒绝大于 255 的值。
**建议：** 按 `u64` 解析。

**审查意见** 审查人: Lightman; 状态: 已接受; 备注:

### GAPTOS-016：ValidatorInfo 中的 `reth_account_address` 缺少长度验证

**文件：** `types/src/validator_info.rs:31`
**问题：** `reth_account_address: Vec<u8>` 没有大小限制。对于以太坊地址应为正好 20 字节。
**建议：** 添加长度验证。

**审查意见** 审查人: Lightman; 状态: 已拒绝; 备注: 长度验证由上游调用方负责

### GAPTOS-017：`GravityExtension` 被排除在 `PartialEq` 之外

**文件：** `types/src/transaction/mod.rs:618`
**问题：** `SignedTransaction::PartialEq` 忽略了 `g_ext`。具有不同区块元数据的两个交易被视为相等。
**建议：** 在相等比较中包含 `g_ext` 或记录有意的排除。

**审查意见** 审查人: Lightman; 状态: 已接受; 备注: 在 `SignedTransaction` 的 `PartialEq` 比较中添加了 `g_ext`

### GAPTOS-018：`GLOBAL_RELAYER.get().unwrap()` 可能 panic

**文件：** `crates/aptos-jwk-consensus/src/jwk_observer.rs:72, 120`
**问题：** `OnceLock` 使用 `.unwrap()` 访问 — 如果中继器未初始化但配置了 `gravity://` 提供者则会 panic。
**建议：** 通过错误日志优雅处理 `None`。

**审查意见** 审查人: Lightman; 状态: 待定; 备注: @AlexYue

### GAPTOS-019：`ValidatorInfoIdl` 转换丢失 `reth_account_address`

**文件：** `types/src/idl/validator_info.rs:80-96`
**问题：** 通过 IDL 格式的往返转换丢失了 reth 账户地址（硬编码为 `vec![]`）。
**建议：** 向 `ValidatorInfoIdl` 添加字段或记录此限制。

**审查意见** 审查人: Lightman; 状态: 已拒绝; 备注: `ValidatorInfoIdl` 目前未使用

### GAPTOS-020：使用 `eprintln!()` 而非结构化日志

**文件：** `types/src/contract_event.rs:41, 55-59`
**问题：** 错误消息使用 `eprintln!()` 而非 `tracing::error!()`。原始公钥字节被打印到 stderr。
**建议：** 使用结构化日志。

**审查意见** 审查人: Lightman; 状态: 已接受; 备注:

### GAPTOS-021：节点配置 HTTPS 字段缺少验证

**文件：** `config/src/config/node_config.rs:90-100`
**问题：** HTTPS 证书/密钥路径没有存在性或路径遍历验证。
**建议：** 添加 `ConfigSanitizer` 验证。

**审查意见** 审查人: Lightman; 状态: 已拒绝; 备注: HTTPS 配置字段仅用于本地测试，不需要验证

### GAPTOS-022：`Bcs::meta()` 包含 `todo!()` — 调用时会 panic

**文件：** `api/src/bcs_payload.rs:70-83`
**问题：** API 响应 meta 实现会 panic。
**建议：** 实现或返回空默认值。

**审查意见** 审查人: Lightman; 状态: 待定; 备注: 无用代码，考虑完全移除

### GAPTOS-023：`new_v2_with_type_tag_str` 在类型标签解析时使用 `unwrap()`

**文件：** `types/src/contract_event.rs:129-134`
**问题：** `TypeTag::from_str(type_tag_str).unwrap()` — 格式错误的类型标签时 panic。
**建议：** 返回 `Result` 或在编译时验证。

**审查意见** 审查人: Lightman; 状态: 待定; 备注: 原始 Aptos 代码

---

## 信息（8）

### GAPTOS-INFO-001：生产代码中的中文注释
**文件：** `types/src/on_chain_config/consensus_config.rs:197-199`
**问题：** 中文内部注释。降低了英语贡献者的可维护性。

**审查意见** 审查人: Lightman; 状态: 已接受; 备注: 已将 `consensus_config.rs` 中的中文注释翻译为英文

### GAPTOS-INFO-002：约200行被注释的 Aptos 代码
**文件：** `types/src/on_chain_config/consensus_config.rs:22-130`
**问题：** 原始类型定义被注释掉。产生噪音。

**审查意见** 审查人: Lightman; 状态: 忽略; 备注:

### GAPTOS-INFO-003：DAG 模块上的 `#![allow(dead_code)]`
**文件：** `consensus/src/dag/mod.rs:3`
**问题：** 模块级死代码抑制。

**审查意见** 审查人: Lightman; 状态: 忽略; 备注:

### GAPTOS-INFO-004：DKG 冒烟测试使用确定性种子
**文件：** `dkg/src/dkg_manager/mod.rs:322-326`
**问题：** 冒烟测试模式下使用确定性 RNG。已正确使用 feature gate。

**审查意见** 审查人: Lightman; 状态: 忽略; 备注:

### GAPTOS-INFO-005：实现了 `Into` trait 而非 `From`
**文件：** `types/src/contract_event.rs:546-550`
**问题：** Rust 惯例更倾向于 `impl From<A> for B` 而非 `impl Into<B> for A`。

**审查意见** 审查人: Lightman; 状态: 已接受; 备注:

### GAPTOS-INFO-006：`JwkIdlError::JsonDeserializationError` 用于 BCS 错误
**文件：** `types/src/idl/jwk_converter.rs:51-61`
**问题：** 错误变体名称写的是 "JSON"，但实际操作是 BCS。

**审查意见** 审查人: Lightman; 状态: 待定; 备注: @AlexYue

### GAPTOS-INFO-007：中继器 `PollResult` 缺少验证文档
**文件：** `crates/api-types/src/relayer.rs:1-27`
**问题：** 没有解释安全性依赖 BFT 法定人数而非单个中继器信任的文档。

**审查意见** 审查人: Lightman; 状态: 待定; 备注: @AlexYue

### GAPTOS-INFO-008：VFN 上游角色扩展
**文件：** `config/src/network_id.rs:183`
**问题：** 通过在上游角色中包含 `NetworkId::Vfn` 启用了 VFN 到 VFN 的同步。扩展了信任边界。

**审查意见** 审查人: Lightman; 状态: 忽略; 备注:
