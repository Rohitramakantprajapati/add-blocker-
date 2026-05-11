use serde::{Deserialize, Serialize};

use crate::{delta::BlocklistDelta, Result};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GossipMessage {
    pub origin_peer: String,
    pub delta: BlocklistDelta,
}

impl GossipMessage {
    pub fn encode(&self) -> Result<Vec<u8>> {
        Ok(bincode::serialize(self)?)
    }

    pub fn decode(data: &[u8]) -> Result<Self> {
        Ok(bincode::deserialize(data)?)
    }
}
