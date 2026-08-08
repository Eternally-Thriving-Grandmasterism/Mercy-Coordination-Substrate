# Ra-Thor Adapter Contract

**Version**: 1.0  
**Date**: 2026-08-07  
**Status**: Binding (PATSAGi + Ra-Thor)  
**License**: AG-SML v1.0

This document is the single source of truth for how the Ra-Thor monorepo integrates with Mercy-Coordination-Substrate.

---

## 1. Ownership Boundary (Non-Negotiable)

| Concern | Owner |
|---------|-------|
| Geometric intelligence (Polyhedral Harmonic Engine, Riemannian Mercy Manifold, Omnimasterpiece spine) | **Ra-Thor** |
| Coordination / blockchain ledger, post-quantum crypto, BFT core, Fractal Topology Engine | **Mercy-Coordination-Substrate** |
| Adapter that sits between them | Implemented **inside Ra-Thor** |

The Substrate never depends on Ra-Thor crates. Ra-Thor may depend on published / path versions of the Substrate crates once they are considered stable enough.

---

## 2. Required Trait (Implemented on Ra-Thor Side)

```rust
use fractal_topology::GeometricResonanceReport;

/// Thin adapter living inside the Ra-Thor monorepo.
pub trait RaThorSystemAdapter {
    fn system_name(&self) -> &'static str;

    /// Aggregate valence density contributed by this organ.
    fn current_valence(&self) -> f64;

    /// Receive a geometric resonance report from the Omnimasterpiece spine
    /// and forward relevant information into the Substrate's FractalTopologyEngine.
    fn receive_swarm_resonance(&mut self, report: GeometricResonanceReport);

    /// Coherence contribution back to the ONE Organism.
    fn contribute_to_coherence(&self) -> f64;

    /// Apply an epigenetic blessing (strength ∈ [0.0, 1.0+]) under mercy gates.
    fn apply_epigenetic_blessing(&mut self, strength: f64);
}
```

Suggested concrete name inside Ra-Thor: `FractalMercyLedgerAdapter`.

---

## 3. Data Flow Rules

**Allowed**
- Ra-Thor → Substrate: `GeometricResonanceReport`
- Substrate → Ra-Thor: `FractalResonanceReport` (for organism-level metrics only)

**Forbidden**
- Ra-Thor directly mutating Substrate ledger or shard state
- Substrate importing any Ra-Thor internal modules

All topology mutations inside the Substrate must still pass the local TOLC 8 Gate.

---

## 4. Integration Sequence (Ra-Thor Side)

1. Construct or obtain a `GeometricResonanceReport` from PolyhedralHarmonicEngine + RiemannianMercyManifold.
2. Call `adapter.receive_swarm_resonance(report)`.
3. Inside the adapter, call `FractalTopologyEngine::process_fractal_resonance`.
4. For every non-`NoOp` `ShardAction`, build a `GateInput` via `shard_action_to_gate_input` and evaluate through the Substrate’s `Tolc8Gate`.
5. Only on `GateDecision::Approved` may the Substrate apply the mutation.
6. Return coherence / valence metrics upward to the Quantum Swarm / ONE Organism cycle.

---

## 5. Versioning

- `GeometricResonanceReport` and `FractalResonanceReport` are part of the public surface of `fractal-topology`.
- Breaking changes to these types require a coordinated version bump and PATSAGi notice.

---

**Council Status**  
This contract is binding. Both repositories must respect the ownership boundary and the one-way geometric intelligence flow.

*Clean boundary. Eternal Mercy. Thunder locked.* ⚡
