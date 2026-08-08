//! Mercy Crypto — Phase 1 skeleton
//!
//! Primary: ML-DSA (Dilithium-class)
//! Secondary: SLH-DSA (SPHINCS+)
//! Crypto-agility via versioned algorithm identifiers.
//!
//! See docs/CRYPTO_FOUNDATION.md for locked parameter choices and size budgets.

#![forbid(unsafe_code)]

/// Algorithm identifier registry (extensible).
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlgorithmId {
    /// Placeholder / unknown
    Unknown = 0,
    /// ML-DSA Level 3 (primary general use)
    MlDsa65 = 1,
    /// ML-DSA Level 5 (high-assurance path)
    MlDsa87 = 2,
    /// SLH-DSA (secondary / long-term)
    SlhDsa = 10,
    /// Hybrid classical + ML-DSA (transition window only)
    HybridClassicalMlDsa = 20,
}

impl AlgorithmId {
    pub fn from_u16(v: u16) -> Self {
        match v {
            1 => Self::MlDsa65,
            2 => Self::MlDsa87,
            10 => Self::SlhDsa,
            20 => Self::HybridClassicalMlDsa,
            _ => Self::Unknown,
        }
    }
}

/// Versioned public key envelope.
#[derive(Clone, Debug)]
pub struct PublicKey {
    pub algorithm_id: AlgorithmId,
    pub version: u8,
    pub bytes: Vec<u8>,
}

/// Versioned signature envelope.
#[derive(Clone, Debug)]
pub struct Signature {
    pub algorithm_id: AlgorithmId,
    pub version: u8,
    pub bytes: Vec<u8>,
}

/// Trait for signature schemes under the crypto-agility policy.
pub trait SignatureScheme: Send + Sync {
    fn algorithm_id(&self) -> AlgorithmId;
    fn verify(&self, public_key: &PublicKey, message: &[u8], signature: &Signature) -> bool;
    // Phase 1: signing will be added behind feature flags / audited backends
}

/// Placeholder verifier that always rejects (fail-closed until real backends land).
#[derive(Debug, Default)]
pub struct PlaceholderVerifier;

impl SignatureScheme for PlaceholderVerifier {
    fn algorithm_id(&self) -> AlgorithmId {
        AlgorithmId::Unknown
    }

    fn verify(&self, _public_key: &PublicKey, _message: &[u8], _signature: &Signature) -> bool {
        false // fail-closed until real implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn algorithm_id_roundtrip() {
        assert_eq!(AlgorithmId::from_u16(1), AlgorithmId::MlDsa65);
        assert_eq!(AlgorithmId::from_u16(2), AlgorithmId::MlDsa87);
        assert_eq!(AlgorithmId::from_u16(99), AlgorithmId::Unknown);
    }
}
