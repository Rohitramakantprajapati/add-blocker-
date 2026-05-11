use serde::{Deserialize, Serialize};

use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlocklistDelta {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub sequence: u64,
}

impl BlocklistDelta {
    pub fn encode(&self) -> Result<Vec<u8>> {
        Ok(bincode::serialize(self)?)
    }

    pub fn decode(data: &[u8]) -> Result<Self> {
        Ok(bincode::deserialize(data)?)
    }
}
