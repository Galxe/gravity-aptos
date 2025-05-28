
use bytes::Bytes;
use std::str::FromStr;


pub enum OnChainConfig {
    ConsensusConfig,
    ExecutionConfig,
    ChainId,
    Configuration,
    ApprovedExecutionHashes,
    Version,
    GasSchedule,
    JWKConsensusConfig,
    RandomnessConfigSeqNum,
    RandomnessConfig,
    CurrentTimeMicroseconds,
    PerBlockRandomness,
    ValidatorSet,
    Epoch,
}

// 实现 FromStr trait 用于从字符串转换为枚举
impl FromStr for OnChainConfig {
    type Err = String; // 定义错误类型，这里简单使用 String

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ConsensusConfig" => Ok(OnChainConfig::ConsensusConfig),
            "ExecutionConfig" => Ok(OnChainConfig::ExecutionConfig),
            "ChainId" => Ok(OnChainConfig::ChainId),
            "Configuration" => Ok(OnChainConfig::Configuration),
            "ApprovedExecutionHashes" => Ok(OnChainConfig::ApprovedExecutionHashes),
            "Version" => Ok(OnChainConfig::Version),
            "GasSchedule" => Ok(OnChainConfig::GasSchedule),
            "JWKConsensusConfig" => Ok(OnChainConfig::JWKConsensusConfig),
            "ValidatorSet" => Ok(OnChainConfig::ValidatorSet),
            "Epoch" => Ok(OnChainConfig::Epoch),
            _ => Err(format!("Unknown OnChainConfig variant: {}", s)),
        }
    }
}

impl TryFrom<String> for OnChainConfig {
    type Error = String; // 通常和 FromStr 的错误类型保持一致

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // 最简洁的方式是利用已经实现的 FromStr trait
        // String 类型可以通过 .parse() 方法调用其 FromStr 实现
        // 或者直接调用 OnChainConfig::from_str(&value)
        value.parse() // 等价于 OnChainConfig::from_str(&value)
    }
}


/// Trait to be implemented by a storage type from which to read on-chain configs
pub trait ConfigStorage : Send + Sync + 'static {
    fn fetch_config_bytes(&self, config_name: OnChainConfig, block_number: u64) -> Option<Bytes>;
}