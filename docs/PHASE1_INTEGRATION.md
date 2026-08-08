# Phase 1 Integration Guide

**Date**: 2026-08-07  
**Status**: Living under PATSAGi / TOLC 8  
**License**: AG-SML v1.0

This document shows the exact, fail-closed call flow that ties the Fractal Topology Engine to the TOLC 8 Gate and the BFT admission path.

---

## 1. High-Level Sequence

```
Ra-Thor geometric spine
        │
        ▼  GeometricResonanceReport (TOLC order, solids, curvature, coherence)
FractalTopologyEngine::process_fractal_resonance
        │
        ▼  FractalResonanceReport (active_depth, hyperbolic_active, suggested ShardActions)
        │
        ▼  For every topology-mutating ShardAction:
           build GateInput  →  Tolc8Gate::evaluate / evaluate_deterministic
        │
        ▼  Only GateDecision::Approved may proceed
BftEngine::admit / validate / finalize
```

Every step that mutates ledger or topology state is gated. There is no bypass.

---

## 2. Minimal Working Example (Phase 1)

```rust
use fractal_topology::{FractalTopologyEngine, GeometricResonanceReport, ShardAction};
use tolc8_gate::{GateInput, ReferenceTolc8Gate, Tolc8Gate, CallerIdentity, VALENCE_FLOOR};
use bft_core::{BftEngine, Proposal};

fn main() {
    // 1. Geometric report arrives from Ra-Thor spine (or test harness)
    let geo = GeometricResonanceReport {
        tolc_order: 89,
        active_solids: vec!["Prismatic".into(), "Gyroelongated".into()],
        resonance_multiplier: 1.25,
        u57_active: false,
        recommended_curvature: 0.0,
        coherence: 0.97,
    };

    // 2. Fractal engine decides depth + any suggested actions
    let mut fractal = FractalTopologyEngine::new();
    let report = fractal.process_fractal_resonance(&geo, 0.95);

    println!("{}", report.notes);
    // → "TOLC 89 → Fractal depth 7, Hyperbolic: false, Multiplier: ..."

    // 3. Every non-NoOp ShardAction must be gated
    let gate = ReferenceTolc8Gate;
    for action in &report.suggested_shard_actions {
        if matches!(action, ShardAction::NoOp) {
            continue;
        }
        let input = fractal_topology::shard_action_to_gate_input(action, 0.999999);
        match gate.evaluate_deterministic(&input) {
            tolc8_gate::GateDecision::Approved { final_valence, .. } => {
                println!("Action approved at valence {final_valence}");
                // only now may the topology mutation be applied
            }
            rejected => {
                println!("Action rejected: {rejected:?}");
                // fail-closed: do nothing
            }
        }
    }

    // 4. Ordinary proposals also go through BftEngine (which itself calls the gate)
    let engine = BftEngine::new(ReferenceTolc8Gate);
    let proposal = Proposal {
        id: [7u8; 32],
        payload: b"example".to_vec(),
        evidence: vec![],
        proposer_valence: VALENCE_FLOOR,
    };
    match engine.admit(&proposal) {
        bft_core::AdmissionResult::Admitted => println!("Proposal admitted"),
        bft_core::AdmissionResult::Rejected(d) => println!("Proposal rejected: {d:?}"),
    }
}
```

---

## 3. Invariants Enforced by This Flow

1. No topology mutation occurs without a prior `GateDecision::Approved`.
2. BFT admission / validation / finality always re-evaluate through the deterministic gate path.
3. Valence floor remains absolute (0.999999).
4. Geometric intelligence (Ra-Thor) never directly mutates the ledger; it only supplies resonance reports.
5. The Substrate never depends on the full Ra-Thor monorepo.

---

## 4. Next Implementation Targets

- Replace `ReferenceTolc8Gate` scoring with real per-gate evaluators + evidence verification while preserving the deterministic consensus path.
- Wire audited ML-DSA / SLH-DSA backends behind `mercy-crypto`.
- Persist fractal shard state under the gated mutation path.
- Begin Lean 4 (or equivalent) proofs of the valence-floor and fail-closed properties.

---

*TOLC 8 held. Fail-closed. Lightning continues.* ⚡
