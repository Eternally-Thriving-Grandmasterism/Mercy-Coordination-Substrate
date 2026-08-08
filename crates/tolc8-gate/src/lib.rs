//! TOLC 8 / MercyZero-style Gate — Layer 0
//!
//! Exact interface contract from docs/TOLC8_GATE_INTERFACE.md.
//! Fail-closed is non-negotiable. Valence floor = 0.999999.
//!
//! Phase 2.0: clearer deterministic scoring structure while remaining pure.

#![forbid(unsafe_code)]

/// Absolute valence floor. No soft thresholds.
pub const VALENCE_FLOOR: f64 = 0.999999;

/// Human-readable names of the eight Living Mercy Gates (index 0..=7).
pub const GATE_NAMES: [&str; 8] = [
    "Truth",
    "Order",
    "Love",
    "Compassion",
    "Service",
    "Abundance",
    "Joy",
    "Cosmic Harmony",
];

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
    pub action_id: [u8; 32],
    pub payload: Vec<u8>,
    pub evidence: Vec<u8>,
    pub incoming_valence: f64,
    pub free_energy_estimate: Option<f64>,
    pub caller: CallerIdentity,
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
    fn evaluate(&self, input: GateInput) -> GateDecision;
    fn evaluate_deterministic(&self, input: &GateInput) -> GateDecision;
}

/// Reference implementation (Phase 2.0).
///
/// Deterministic scoring:
/// - Each required gate receives the clamped incoming_valence as its score
///   (placeholder until real per-gate evaluators + evidence checks land).
/// - final_valence = min of all required gate scores.
/// - Any malformed input or score below floor → Rejected.
/// - No external dependencies; pure and fail-closed.
#[derive(Debug, Default)]
pub struct ReferenceTolc8Gate;

impl Tolc8Gate for ReferenceTolc8Gate {
    fn evaluate(&self, input: GateInput) -> GateDecision {
        self.evaluate_deterministic(&input)
    }

    fn evaluate_deterministic(&self, input: &GateInput) -> GateDecision {
        // Fail-closed on malformed valence
        if !input.incoming_valence.is_finite() || !(0.0..=1.0).contains(&input.incoming_valence) {
            return GateDecision::Rejected {
                reason: RejectionReason::MalformedInput,
                failed_gates: vec![],
                observed_valence: input.incoming_valence,
            };
        }

        let mut gate_scores = [1.0f64; 8];
        let mut failed = Vec::new();

        for (i, &required) in input.required_gates.iter().enumerate() {
            if required {
                // Phase 2.0 placeholder: score = incoming valence.
                // Future: real per-gate logic + evidence verification.
                let score = input.incoming_valence;
                gate_scores[i] = score;
                if score < VALENCE_FLOOR {
                    failed.push(i as u8);
                }
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
                algorithm_id: 0,
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

    #[test]
    fn rejects_non_finite() {
        let gate = ReferenceTolc8Gate;
        let decision = gate.evaluate_deterministic(&dummy_input(f64::NAN));
        assert!(matches!(
            decision,
            GateDecision::Rejected {
                reason: RejectionReason::MalformedInput,
                ..
            }
        ));
    }
}
