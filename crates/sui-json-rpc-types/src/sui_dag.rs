use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use consensus_core::{BlockAPI, VerifiedBlock};
use base64::{engine::general_purpose, Engine};

/// Basic information about a DAG block.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SuiDagBlock {
    /// Epoch when this block was produced.
    #[schemars(with = "u64")]
    pub epoch: u64,
    /// Round of the block.
    pub round: u32,
    /// Numeric identifier of the authoring validator.
    pub author: u32,
    /// Base64 encoded digest of the block.
    pub digest: String,
    /// Timestamp in milliseconds.
    #[schemars(with = "u64")]
    pub timestamp_ms: u64,
    /// Digests of parent blocks (base64 encoded).
    pub parents: Vec<String>,
}

impl From<VerifiedBlock> for SuiDagBlock {
    fn from(block: VerifiedBlock) -> Self {
        SuiDagBlock {
            epoch: block.epoch() as u64,
            round: block.round(),
            author: block.author().value() as u32,
            digest: general_purpose::STANDARD.encode(block.reference().digest.as_ref()),
            timestamp_ms: block.timestamp_ms(),
            parents: block
                .ancestors()
                .iter()
                .map(|p| {
                    general_purpose::STANDARD.encode(p.digest.as_ref())
                })
                .collect(),
        }
    }
}
