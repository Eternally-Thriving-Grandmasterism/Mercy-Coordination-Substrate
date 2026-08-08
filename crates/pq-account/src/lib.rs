//! PQ Account — Phase 1 skeleton
//!
//! Account model that supports efficient post-quantum key rotation
//! without forcing asset movement.

#![forbid(unsafe_code)]

use mercy_crypto::{AlgorithmId, PublicKey};

/// Account identifier (32-byte opaque).
pub type AccountId = [u8; 32];

/// A single authorized public key with its algorithm and validity window.
#[derive(Clone, Debug)]
pub struct AuthorizedKey {
    pub public_key: PublicKey,
    pub valid_from: u64, // logical timestamp / epoch
    pub valid_until: Option<u64>,
}

/// Account state (minimal).
#[derive(Clone, Debug)]
pub struct Account {
    pub id: AccountId,
    pub keys: Vec<AuthorizedKey>,
    /// Sequence number for replay protection / rotation ordering
    pub sequence: u64,
}

impl Account {
    pub fn new(id: AccountId) -> Self {
        Self {
            id,
            keys: Vec::new(),
            sequence: 0,
        }
    }

    /// Rotate / add a key without moving assets.
    /// Real authorization and gate checks occur at the ledger / gate layer.
    pub fn authorize_key(&mut self, key: AuthorizedKey) {
        self.keys.push(key);
        self.sequence = self.sequence.saturating_add(1);
    }

    /// Return currently valid keys for a given epoch (skeleton).
    pub fn valid_keys_at(&self, epoch: u64) -> Vec<&AuthorizedKey> {
        self.keys
            .iter()
            .filter(|k| k.valid_from <= epoch && k.valid_until.map(|u| epoch <= u).unwrap_or(true))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mercy_crypto::AlgorithmId;

    #[test]
    fn key_rotation_increments_sequence() {
        let mut acc = Account::new([1u8; 32]);
        assert_eq!(acc.sequence, 0);
        acc.authorize_key(AuthorizedKey {
            public_key: PublicKey {
                algorithm_id: AlgorithmId::MlDsa65,
                version: 1,
                bytes: vec![0u8; 32],
            },
            valid_from: 0,
            valid_until: None,
        });
        assert_eq!(acc.sequence, 1);
    }
}
