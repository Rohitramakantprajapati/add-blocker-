use ed25519_dalek::{Signature, Verifier, VerifyingKey};

use crate::{Result, VoidBlockError};

pub fn verify_signature(public_key: &[u8; 32], message: &[u8], signature: &[u8; 64]) -> Result<()> {
    let key = VerifyingKey::from_bytes(public_key).map_err(|error| VoidBlockError::Crypto(error.to_string()))?;
    let signature = Signature::from_bytes(signature);
    key.verify(message, &signature).map_err(|error| VoidBlockError::Signature(error.to_string()))?;
    Ok(())
}
