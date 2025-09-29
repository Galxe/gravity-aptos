use serde::{Deserialize, Serialize};
use crate::account::ExternalAccountAddress;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
/// DKG transcript and its metadata.
pub struct DKGTranscript {
    pub metadata: DKGTranscriptMetadata,
    #[serde(with = "serde_bytes")]
    pub transcript_bytes: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct DKGTranscriptMetadata {
    pub epoch: u64,
    pub author: ExternalAccountAddress,
}
