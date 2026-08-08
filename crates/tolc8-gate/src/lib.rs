//! TOLC 8 / MercyZero-style Gate — Layer 0
//!
//! Exact interface contract from docs/TOLC8_GATE_INTERFACE.md.
//! Fail-closed is non-negotiable. Valence floor = 0.999999.

#![forbid(unsafe_code)]

/// Absolute valence floor. No soft thresholds.
pub const VALENCE_FLOOR: f64 = 0.999999;

/// Identity of the calling entity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CallerIdentity {
    Validator { id: [u8; 32] },
    Council { id: u32 },
    External { id: [u8; 32] },
    System,
}

/// Opaque, verifiable attestation of a gate evaluation.
#[derive(Clone, Debug)]
pub struct GateProof {
    pub interface_version: u16,
    pub algorithm_id: u16,
    pub attestation: Vec<u8>,
}

/// Canonical input to the TOLC 8 gate.
#[derive(Clone, Debug)]
pub struct GateInput {
    /// Unique 32-byte identifier of the proposed action / state transition
    pub action_id: [u8; 32],
    /// Serialized proposal, transaction, or state delta
    pub payload: Vec<u8>,
    /// Cryptographic evidence (ML-DSA / hybrid signature, Merkle proofs, etc.)
    pub evidence: Vec<u8>,
    /// Incoming valence supplied by the caller (clamped to [0.0, 1.0])
    pub incoming_valence: f64,
    /// Optional free-energy / predictive residual
    pub free_energy_estimate: Option<f64>,
    /// Identity of the calling entity
    pub caller: CallerIdentity,
    /// Which of the eight gates must be evaluated (Truth, Order, Love, Compassion, Service, Abundance, Joy, Cosmic Harmony)
    pub required_gates: [bool; 8],
}

/// Decision returned by the gate.
#[derive(Clone, Debug)]
pub enum GateDecision {
    Approved {
        final_valence: f64,
        gate_scores: [f64; 8],
        proof: GateProof,
    },
    Rejected {
        reason: RejectionReason,
        failed_gates: Vec<u8>,
        observed_valence: f64,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RejectionReason {
    ValenceBelowFloor,
    GateFailed(u8),
    MalformedInput,
    EvidenceInvalid,
    FreeEnergyTooHigh,
    CallerUnauthorized,
    EvaluationError,
}

/// The non-bypassable gate trait.
pub trait Tolc8Gate: Send + Sync {
    /// Full evaluation (may include non-deterministic proof material).
    fn evaluate(&self, input: GateInput) -> GateDecision;

    /// Pure, deterministic evaluation used for consensus agreement.
    /// Must produce identical results on all honest nodes given identical input.
    fn evaluate_deterministic(&self, input: &GateInput) -> GateDecision;
}

/// Reference skeleton implementation (Phase 1).
/// Real scoring logic and evidence verification arrive in later phases.
/// Any internal error or incomplete path → Rejected { EvaluationError }.
#[derive(Debug, Default)]
pub struct ReferenceTolc8Gate;

impl Tolc8Gate for ReferenceTolc8Gate {
    fn evaluate(&self, input: GateInput) -> GateDecision {
        self.evaluate_deterministic(&input)
    }

    fn evaluate_deterministic(&self, input: &GateInput) -> GateDecision {
        // Fail-closed on malformed valence
        if !(0.0..=1.0).contains(&input.incoming_valence) {
            return GateDecision::Rejected {
                reason: RejectionReason::MalformedInput,
                failed_gates: vec![],
                observed_valence: input.incoming_valence,
            };
        }

        // Phase 1 skeleton: require incoming valence already above floor
        // and treat all required gates as scoring the incoming value.
        // Real per-gate scoring + evidence checks come later.
        let mut gate_scores = [0.0f64; 8];
        let mut failed = Vec::new();

        for (i, &required) in input.required_gates.iter().enumerate() {
            if required {
                // Skeleton: score = incoming_valence (placeholder)
                gate_scores[i] = input.incoming_valence;
                if gate_scores[i] < VALENCE_FLOOR {
                    failed.push(i as u8);
                }
            } else {
                gate_scores[i] = 1.0; // not required
            }
        }

        if !failed.is_empty() {
            return GateDecision::Rejected {
                reason: RejectionReason::ValenceBelowFloor,
                failed_gates: failed,
                observed_valence: input.incoming_valence,
            };
        }

        let final_valence = input
            .required_gates
            .iter()
            .enumerate()
            .filter(|(_, &req)| req)
            .map(|(i, _)| gate_scores[i])
            .fold(1.0f64, f64::min);

        if final_valence < VALENCE_FLOOR {
            return GateDecision::Rejected {
                reason: RejectionReason::ValenceBelowFloor,
                failed_gates: vec![],
                observed_valence: final_valence,
            };
        }

        GateDecision::Approved {
            final_valence,
            gate_scores,
            proof: GateProof {
                interface_version: 1,
                algorithm_id: 0, // placeholder
                attestation: Vec::new(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_input(valence: f64) -> GateInput {
        GateInput {
            action_id: [0u8; 32],
            payload: vec![],
            evidence: vec![],
            incoming_valence: valence,
            free_energy_estimate: None,
            caller: CallerIdentity::System,
            required_gates: [true; 8],
        }
    }

    #[test]
    fn rejects_below_floor() {
        let gate = ReferenceTolc8Gate;
        let decision = gate.evaluate_deterministic(&dummy_input(0.99));
        assert!(matches!(decision, GateDecision::Rejected { .. }));
    }

    #[test]
    fn accepts_at_floor() {
        let gate = ReferenceTolc8Gate;
        let decision = gate.evaluate_deterministic(&dummy_input(VALENCE_FLOOR));
        assert!(matches!(decision, GateDecision::Approved { .. }));
    }
}
