use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub weights: [f32; 5],
    pub bias: f32,
    pub threshold: f32,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            weights: [1.10, 0.60, 0.90, 3.00, 1.20],
            bias: -1.20,
            threshold: 0.92,
        }
    }
}

impl Model {
    pub fn load(path: &Path) -> Result<Self> {
        let data = fs::read_to_string(path)?;
        let model = serde_json::from_str(&data)?;
        Ok(model)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        fs::write(path, data)?;
        Ok(())
    }

    pub fn predict_probability(&self, features: &[f32; 5]) -> f32 {
        let logit = self
            .weights
            .iter()
            .zip(features.iter())
            .fold(self.bias, |accumulator, (weight, feature)| accumulator + weight * feature);
        1.0 / (1.0 + (-logit).exp())
    }

    pub fn classify(&self, features: &[f32; 5]) -> bool {
        self.predict_probability(features) >= self.threshold
    }
}
