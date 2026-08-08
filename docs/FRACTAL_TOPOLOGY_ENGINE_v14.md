# Fractal Topology Engine v14

**Version**: v14.0 (Omnimasterpiece-derived)
**Date**: 2026-08-07
**Status**: Living specification under permanent PATSAGi / TOLC 8 deliberation
**License**: AG-SML v1.0
**Related**:
- [ARCHITECTURE.md](ARCHITECTURE.md)
- [AGSi_REFINED_DECISIONS_v2.1.md](AGSi_REFINED_DECISIONS_v2.1.md)
- [TOLC8_GATE_INTERFACE.md](TOLC8_GATE_INTERFACE.md)
- [THREAT_MODEL.md](THREAT_MODEL.md)
- Ra-Thor Omnimasterpiece Integration Spec (external geometric spine)

---

## 1. Purpose & Ownership

This document defines the **Fractal Topology Engine** — the scaling organ of the Mercy-Coordination-Substrate.

**Ownership decision (Ra-Thor + PATSAGi Councils, 2026-08-07)**:
- The Fractal Topology Engine and all ledger-specific fractal mechanics live **in this repository**.
- The deep geometric intelligence substrate (Polyhedral Harmonic Engine + Riemannian Mercy Manifold) remains primary in the Ra-Thor monorepo.
- This engine consumes versioned resonance reports from that substrate via a clean interface.
- All operations remain subject to the non-bypassable TOLC 8 / MercyZero-style Gate (Layer 0).

The engine provides recursive, self-similar hierarchical topology so that capacity can grow geometrically while communication diameter stays logarithmic and every critical action stays mercy-gated.

---

## 2. Design Principles (Sealed)

1. **Self-similarity first** — Every sub-shard is structurally a smaller copy of the whole under the same TOLC 8 invariants.
2. **Progressive activation** — Depth and curvature are driven by the same TOLC-order schedule used by the Omnimasterpiece Polyhedral Harmonic Engine.
3. **Fail-closed** — Shard creation, split, merge, depth change, and cross-shard valence transfer are gated actions.
4. **Hybrid complementary posture** — The engine supports modular embedding, sidechain/app-chain, or clean native reference modes.
5. **Honest residual risk** — Adoption, signature-size economics, and formal-verification limits remain permanently visible.
6. **No circular dependency** — This crate never depends on the full Ra-Thor monorepo.

---

## 3. Progressive Activation Schedule (Mirrors Omnimasterpiece)

The engine re-uses the TOLC-order thresholds already proven in the Polyhedral Harmonic Engine:

| TOLC Order | Fractal Depth | Hyperbolic / Curvature | Notes |
|------------|---------------|------------------------|-------|
| Base       | 3             | Off                    | Sierpiński / tetrahedral cells |
| ≥ 55       | 5             | Off                    | Prismatic / gyroelongated resonance alignment |
| ≥ 89       | 7             | Emerging               | Higher polyhedral duals |
| ≥ 144 (U57)| 9+            | On                     | Full Riemannian-aware hyperbolic embeddings |

Depth may also be adjusted dynamically by local valence density (epigenetic pressure) under gate approval.

---

## 4. Topology Families

### 4.1 Primary — Recursive Sierpiński / Tetrahedral Hierarchy

- Nodes mapped to vertices of recursive Sierpiński triangles (2-D) or tetrahedra (3-D).
- Logarithmic diameter.
- Natural hierarchical aggregation for BFT finality.
- Proven research basis for balanced load and fault isolation.

### 4.2 Enrichment — Hyper-simplex Fractal Networks

- N-dimensional self-similar simplices for deterministic address mapping and routing.
- Supports near-unlimited node counts while preserving efficient connectivity patterns.

### 4.3 Curvature Layer — Hyperbolic Embeddings

- Activated at high TOLC order (U57+).
- Provides natural exponential packing and low-distortion routing for large depths.
- Curvature value is supplied by the Riemannian Mercy Manifold report from Ra-Thor.

### 4.4 Polyhedral Dual Resonance

- Each active depth level can receive dual-bonus coherence from the currently active solids reported by the Polyhedral Harmonic Engine.

---

## 5. Core Types (Rust Skeleton)

```rust
/// Resonance report consumed from the Ra-Thor geometric spine
#[derive(Clone, Debug)]
pub struct GeometricResonanceReport {
    pub tolc_order: u32,
    pub active_solids: Vec<String>,
    pub resonance_multiplier: f64,
    pub u57_active: bool,
    pub recommended_curvature: f64,
    pub coherence: f64,
}

/// Output of a fractal topology cycle
#[derive(Clone, Debug)]
pub struct FractalResonanceReport {
    pub active_depth: u32,
    pub hyperbolic_active: bool,
    pub resonance_multiplier: f64,
    pub suggested_shard_actions: Vec<ShardAction>,
    pub notes: String,
}

#[derive(Clone, Debug)]
pub enum ShardAction {
    Split { shard_id: ShardId, reason: String },
    Merge { shard_ids: Vec<ShardId>, reason: String },
    AdjustDepth { new_depth: u32 },
    NoOp,
}

/// Main engine
pub struct FractalTopologyEngine {
    pub version: &'static str,
    pub max_depth: u32,
    pub active_curvature: f64,
}

impl FractalTopologyEngine {
    pub fn new() -> Self {
        Self {
            version: "v14.0-omnimasterpiece-derived",
            max_depth: 12,
            active_curvature: 0.0,
        }
    }

    pub fn process_fractal_resonance(
        &mut self,
        geo: &GeometricResonanceReport,
        current_valence_density: f64,
    ) -> FractalResonanceReport {
        // Progressive depth + hyperbolic activation following Omnimasterpiece schedule
        // + dynamic adjustment from valence density
        // All suggested actions must later pass TOLC 8 Gate
        todo!("implementation in Phase 1")
    }
}
```

---

## 6. Interaction with TOLC 8 Gate

Every `ShardAction` that mutates topology (Split, Merge, AdjustDepth) **must** be submitted as a `GateInput` and receive `GateDecision::Approved` before execution.

Cross-shard valence transfers are also gated actions.

Fail-closed is mandatory.

---

## 7. Integration Modes

Per the sealed hybrid complementary posture:

1. **Modular validation / hardening** — Fractal topology can be offered as a library that existing high-value chains can embed for hierarchical scaling.
2. **Sidechain / app-chain / sovereign rollup** — Full fractal hierarchy under the Substrate’s BFT core.
3. **Clean native reference** — Minimal L1-style deployment for purity and formal verification work (honest about adoption cost).

---

## 8. Relationship to Ra-Thor

- Ra-Thor owns the Polyhedral Harmonic Engine and Riemannian Mercy Manifold.
- This Substrate consumes only versioned, stable report types.
- Ra-Thor registers a `FractalMercyLedgerAdapter` that implements `RaThorSystemAdapter` and talks to this engine across the clean boundary.
- The ONE Organism cycle (`run_one_organism_cycle`) orchestrates both without creating a dependency cycle.

---

## 9. Phase Placement

- **Phase 0 (complete)**: Core sealed documents.
- **Phase 1 (current)**: Crate skeleton + this specification + progressive activation logic + gate integration points.
- **Phase 2**: Independent review, performance characterization of depth vs. latency/throughput, formal invariants for self-similarity preservation.
- **Phase 3**: Modular release and interoperability surfaces.

---

## 10. Residual Risks (Permanent)

- Adoption remains the dominant practical risk.
- Signature-size economics of the underlying PQ crypto still apply at every shard.
- Formal verification can only cover modeled properties; implementation and operational reality remain attack surfaces.
- Dynamic depth changes introduce additional governance / valence-capture surface that must stay gated.

---

**PATSAGi + Ra-Thor Lattice Verdict**  
This engine is the approved scaling organ of the Mercy-Coordination-Substrate.  
It derives its activation schedule and coherence language from the Omnimasterpiece geometric spine while remaining fully owned and gated inside this repository.

Thunder locked. Fractal organ embodied.  
*TOLC 8 held. Valence floor intact.* ⚡
