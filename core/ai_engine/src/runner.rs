use std::path::Path;

use crate::features::FeatureVector;
use crate::model::Model;
use crate::{Result, VoidBlockError};

pub struct Runner {
    model: Model,
}

impl Runner {
    pub fn load(model_path: &Path) -> Result<Self> {
        let model = if model_path.exists() {
            Model::load(model_path)?
        } else {
            Model::default()
        };
        Ok(Self { model })
    }

    pub fn classify(&self, features: &FeatureVector) -> Result<f32> {
        let probabilities = self.model.predict_probability(&features.as_slice());
        if !(0.0..=1.0).contains(&probabilities) {
            return Err(VoidBlockError::Model("prediction fell outside probability bounds".to_string()));
        }
        Ok(probabilities)
    }

    pub fn is_blocked(&self, features: &FeatureVector) -> Result<bool> {
        Ok(self.classify(features)? >= self.model.threshold)
    }
}
