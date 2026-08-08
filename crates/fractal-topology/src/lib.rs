//! Fractal Topology Engine v14
//!
//! Recursive, self-similar hierarchical scaling organ of the
//! Mercy-Coordination-Substrate.
//!
//! Derived from the Omnimasterpiece progressive activation schedule
//! (Polyhedral Harmonic + Riemannian Mercy Manifold) while remaining
//! fully owned and gated inside this repository.
//!
//! All topology-mutating actions must pass the TOLC 8 / MercyZero-style Gate.
//!
//! See: docs/FRACTAL_TOPOLOGY_ENGINE_v14.md

#![forbid(unsafe_code)]

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
#[derive(Clone, Debug)]
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

    /// Determine active fractal depth and curvature from a geometric resonance report
    /// and current valence density. All suggested actions remain subject to the
    /// TOLC 8 Gate (fail-closed).
    pub fn process_fractal_resonance(
        &mut self,
        geo: &GeometricResonanceReport,
        current_valence_density: f64,
    ) -> FractalResonanceReport {
        let mut depth = 3u32;
        let mut hyperbolic_active = false;
        let mut multiplier = geo.resonance_multiplier;

        // Progressive activation mirroring Omnimasterpiece Polyhedral schedule
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

        // Epigenetic pressure from valence density (still gated later)
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

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!((engine.active_curvature - 0.85).abs() < f64::EPSILON);
    }
}
