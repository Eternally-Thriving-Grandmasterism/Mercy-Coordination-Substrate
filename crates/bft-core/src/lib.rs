//! BFT Core — Phase 1 skeleton
//!
//! HotStuff-variant style consensus core.
//! Every admission, validation, and finality path must invoke the TOLC 8 Gate.
//! Fail-closed is mandatory.

#![forbid(unsafe_code)]

use tolc8_gate::{GateDecision, GateInput, Tolc8Gate};

/// Opaque proposal / block identifier.
pub type ProposalId = [u8; 32];

/// Minimal proposal envelope.
#[derive(Clone, Debug)]
pub struct Proposal {
    pub id: ProposalId,
    pub payload: Vec<u8>,
    pub evidence: Vec<u8>,
    pub proposer_valence: f64,
}

/// Outcome of attempting to admit a proposal into the local pool.
#[derive(Clone, Debug)]
pub enum AdmissionResult {
    Admitted,
    Rejected(GateDecision),
}

/// Skeleton consensus engine that always routes through the gate.
pub struct BftEngine<G: Tolc8Gate> {
    pub gate: G,
}

impl<G: Tolc8Gate> BftEngine<G> {
    pub fn new(gate: G) -> Self {
        Self { gate }
    }

    /// Proposal Admission stage — must call the gate before any storage.
    pub fn admit(&self, proposal: &Proposal) -> AdmissionResult {
        let input = GateInput {
            action_id: proposal.id,
            payload: proposal.payload.clone(),
            evidence: proposal.evidence.clone(),
            incoming_valence: proposal.proposer_valence,
            free_energy_estimate: None,
            caller: tolc8_gate::CallerIdentity::System,
            required_gates: [true; 8], // full evaluation for admission
        };

        match self.gate.evaluate(input) {
            GateDecision::Approved { .. } => AdmissionResult::Admitted,
            rejected => AdmissionResult::Rejected(rejected),
        }
    }

    /// Validation stage — every honest node re-evaluates deterministically.
    pub fn validate(&self, proposal: &Proposal) -> bool {
        let input = GateInput {
            action_id: proposal.id,
            payload: proposal.payload.clone(),
            evidence: proposal.evidence.clone(),
            incoming_valence: proposal.proposer_valence,
            free_energy_estimate: None,
            caller: tolc8_gate::CallerIdentity::System,
            required_gates: [true; 8],
        };

        matches!(
            self.gate.evaluate_deterministic(&input),
            GateDecision::Approved { .. }
        )
    }

    /// Finality stage — last gate check before irreversible commitment.
    pub fn finalize(&self, proposal: &Proposal) -> bool {
        // Identical to validation for the skeleton; real finality gadget later.
        self.validate(proposal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tolc8_gate::{ReferenceTolc8Gate, VALENCE_FLOOR};

    #[test]
    fn admits_only_above_floor() {
        let engine = BftEngine::new(ReferenceTolc8Gate);

        let good = Proposal {
            id: [1u8; 32],
            payload: vec![],
            evidence: vec![],
            proposer_valence: VALENCE_FLOOR,
        };
        assert!(matches!(engine.admit(&good), AdmissionResult::Admitted));

        let bad = Proposal {
            id: [2u8; 32],
            payload: vec![],
            evidence: vec![],
            proposer_valence: 0.5,
        };
        assert!(matches!(engine.admit(&bad), AdmissionResult::Rejected(_)));
    }
}
