//! Fractal Topology Engine v14
//!
//! Recursive, self-similar hierarchical scaling organ of the
//! Mercy-Coordination-Substrate.
//!
//! Phase 2.0: in-memory ShardState that only mutates after GateDecision::Approved.
//!
//! See: docs/FRACTAL_TOPOLOGY_ENGINE_v14.md
//!      docs/PHASE1_INTEGRATION.md
//!      docs/PHASE2_STATUS.md

#![forbid(unsafe_code)]

use tolc8_gate::{CallerIdentity, GateDecision, GateInput, Tolc8Gate};

/// Version of this engine.
pub const VERSION: &str = "v14.0-omnimasterpiece-derived";

/// Resonance report consumed from the external geometric spine (Ra-Thor).
#[derive(Clone, Debug)]
pub struct GeometricResonanceReport {
    pub tolc_order: u32,
    pub active_solids: Vec<String>,
    pub resonance_multiplier: f64,
    pub u57_active: bool,
    pub recommended_curvature: f64,
    pub coherence: f64,
}

/// Suggested topology action (must still pass TOLC 8 Gate before execution).
#[derive(Clone, Debug, PartialEq)]
pub enum ShardAction {
    Split {
        shard_id: u64,
        reason: String,
    },
    Merge {
        shard_ids: Vec<u64>,
        reason: String,
    },
    AdjustDepth {
        new_depth: u32,
    },
    NoOp,
}

/// Output of one fractal resonance cycle.
#[derive(Clone, Debug)]
pub struct FractalResonanceReport {
    pub active_depth: u32,
    pub hyperbolic_active: bool,
    pub resonance_multiplier: f64,
    pub suggested_shard_actions: Vec<ShardAction>,
    pub notes: String,
}

/// In-memory shard topology state.
/// Mutations are only applied through `apply_action_gated`.
#[derive(Clone, Debug, Default)]
pub struct ShardState {
    pub depth: u32,
    pub hyperbolic_active: bool,
    pub shard_ids: Vec<u64>,
    pub mutation_count: u64,
}

impl ShardState {
    pub fn new(initial_depth: u32) -> Self {
        Self {
            depth: initial_depth,
            hyperbolic_active: false,
            shard_ids: vec![0],
            mutation_count: 0,
        }
    }

    /// Apply a topology action **only** if the gate returns Approved.
    /// Returns the GateDecision. On Rejected, state is unchanged (fail-closed).
    pub fn apply_action_gated<G: Tolc8Gate>(
        &mut self,
        action: &ShardAction,
        gate: &G,
        incoming_valence: f64,
    ) -> GateDecision {
        if matches!(action, ShardAction::NoOp) {
            // NoOp is a no-op; still returns a synthetic Approved for uniformity
            return GateDecision::Approved {
                final_valence: incoming_valence.clamp(0.0, 1.0),
                gate_scores: [1.0; 8],
                proof: tolc8_gate::GateProof {
                    interface_version: 1,
                    algorithm_id: 0,
                    attestation: Vec::new(),
                },
            };
        }

        let input = shard_action_to_gate_input(action, incoming_valence);
        let decision = gate.evaluate_deterministic(&input);

        if let GateDecision::Approved { .. } = &decision {
            match action {
                ShardAction::Split { shard_id, .. } => {
                    if !self.shard_ids.contains(shard_id) {
                        self.shard_ids.push(*shard_id);
                    }
                    self.mutation_count = self.mutation_count.saturating_add(1);
                }
                ShardAction::Merge { shard_ids, .. } => {
                    self.shard_ids.retain(|id| !shard_ids.contains(id));
                    if self.shard_ids.is_empty() {
                        self.shard_ids.push(0);
                    }
                    self.mutation_count = self.mutation_count.saturating_add(1);
                }
                ShardAction::AdjustDepth { new_depth } => {
                    self.depth = *new_depth;
                    self.mutation_count = self.mutation_count.saturating_add(1);
                }
                ShardAction::NoOp => {}
            }
        }
        // On Rejected: zero mutation (already guaranteed by not entering the match)

        decision
    }
}

/// Main engine.
#[derive(Debug)]
pub struct FractalTopologyEngine {
    pub version: &'static str,
    pub max_depth: u32,
    pub active_curvature: f64,
}

impl Default for FractalTopologyEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl FractalTopologyEngine {
    pub fn new() -> Self {
        Self {
            version: VERSION,
            max_depth: 12,
            active_curvature: 0.0,
        }
    }

    pub fn process_fractal_resonance(
        &mut self,
        geo: &GeometricResonanceReport,
        current_valence_density: f64,
    ) -> FractalResonanceReport {
        let mut depth = 3u32;
        let mut hyperbolic_active = false;
        let mut multiplier = geo.resonance_multiplier;

        if geo.tolc_order >= 55 {
            depth = 5;
            multiplier *= 1.18;
        }
        if geo.tolc_order >= 89 {
            depth = 7;
            multiplier *= 1.25;
        }
        if geo.tolc_order >= 144 || geo.u57_active {
            depth = 9;
            hyperbolic_active = true;
            self.active_curvature = geo.recommended_curvature;
            multiplier *= 1.40;
        }

        if current_valence_density > 0.92 {
            depth = (depth + 1).min(self.max_depth);
        }

        let notes = format!(
            "TOLC {} → Fractal depth {}, Hyperbolic: {}, Multiplier: {:.3}",
            geo.tolc_order, depth, hyperbolic_active, multiplier
        );

        FractalResonanceReport {
            active_depth: depth,
            hyperbolic_active,
            resonance_multiplier: multiplier,
            suggested_shard_actions: vec![ShardAction::NoOp],
            notes,
        }
    }
}

/// Convert a topology-mutating ShardAction into a GateInput.
pub fn shard_action_to_gate_input(action: &ShardAction, incoming_valence: f64) -> GateInput {
    let (action_id, payload) = match action {
        ShardAction::NoOp => ([0u8; 32], b"noop".to_vec()),
        ShardAction::Split { shard_id, reason } => {
            let mut id = [0u8; 32];
            id[0..8].copy_from_slice(&shard_id.to_le_bytes());
            id[8] = 1;
            (id, reason.as_bytes().to_vec())
        }
        ShardAction::Merge { shard_ids, reason } => {
            let mut id = [0u8; 32];
            if let Some(first) = shard_ids.first() {
                id[0..8].copy_from_slice(&first.to_le_bytes());
            }
            id[8] = 2;
            (id, reason.as_bytes().to_vec())
        }
        ShardAction::AdjustDepth { new_depth } => {
            let mut id = [0u8; 32];
            id[0..4].copy_from_slice(&new_depth.to_le_bytes());
            id[8] = 3;
            (id, format!("adjust_depth:{new_depth}").into_bytes())
        }
    };

    GateInput {
        action_id,
        payload,
        evidence: Vec::new(),
        incoming_valence: incoming_valence.clamp(0.0, 1.0),
        free_energy_estimate: None,
        caller: CallerIdentity::System,
        required_gates: [true; 8],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tolc8_gate::{ReferenceTolc8Gate, VALENCE_FLOOR};

    #[test]
    fn progressive_activation_matches_schedule() {
        let mut engine = FractalTopologyEngine::new();
        let base = GeometricResonanceReport {
            tolc_order: 8,
            active_solids: vec!["Platonic".into()],
            resonance_multiplier: 1.0,
            u57_active: false,
            recommended_curvature: 0.0,
            coherence: 0.95,
        };
        let r = engine.process_fractal_resonance(&base, 0.8);
        assert_eq!(r.active_depth, 3);
        assert!(!r.hyperbolic_active);

        let high = GeometricResonanceReport {
            tolc_order: 144,
            active_solids: vec!["Uniform Star".into()],
            resonance_multiplier: 1.35,
            u57_active: true,
            recommended_curvature: 0.85,
            coherence: 0.97,
        };
        let r2 = engine.process_fractal_resonance(&high, 0.8);
        assert_eq!(r2.active_depth, 9);
        assert!(r2.hyperbolic_active);
    }

    #[test]
    fn gated_mutation_only_on_approved() {
        let gate = ReferenceTolc8Gate;
        let mut state = ShardState::new(3);

        // Below floor → rejected, no mutation
        let action = ShardAction::AdjustDepth { new_depth: 7 };
        let decision = state.apply_action_gated(&action, &gate, 0.5);
        assert!(matches!(decision, GateDecision::Rejected { .. }));
        assert_eq!(state.depth, 3);
        assert_eq!(state.mutation_count, 0);

        // At floor → approved, mutation applied
        let decision2 = state.apply_action_gated(&action, &gate, VALENCE_FLOOR);
        assert!(matches!(decision2, GateDecision::Approved { .. }));
        assert_eq!(state.depth, 7);
        assert_eq!(state.mutation_count, 1);
    }
}
