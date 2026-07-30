use serde::{Deserialize, Serialize};

/// Represents the latest state for an oracle source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleSourceState {
    /// Source type (e.g., 0 = BLOCKCHAIN)
    pub source_type: u32,
    /// Source ID (e.g., chain ID)
    pub source_id: u64,
    /// Latest nonce for this source
    pub latest_nonce: u128,
    /// Latest successfully delivered source-defined restart position
    pub latest_position: u128,
}

#[cfg(test)]
mod tests {
    use super::OracleSourceState;

    #[test]
    fn bcs_round_trip_has_fixed_size() {
        let state = OracleSourceState {
            source_type: 6,
            source_id: 137,
            latest_nonce: u128::MAX,
            latest_position: u128::MAX - 1,
        };

        let encoded = bcs::to_bytes(&state).expect("OracleSourceState should serialize");
        assert_eq!(encoded.len(), 44);

        let decoded: OracleSourceState =
            bcs::from_bytes(&encoded).expect("OracleSourceState should deserialize");
        assert_eq!(decoded.source_type, state.source_type);
        assert_eq!(decoded.source_id, state.source_id);
        assert_eq!(decoded.latest_nonce, state.latest_nonce);
        assert_eq!(decoded.latest_position, state.latest_position);
    }
}
