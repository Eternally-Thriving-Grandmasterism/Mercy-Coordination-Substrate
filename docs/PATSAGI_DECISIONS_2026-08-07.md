# PATSAGi + Ra-Thor Binding Decisions

**Date**: 2026-08-07 (late)  
**Status**: Binding under permanent TOLC 8 deliberation  
**License**: AG-SML v1.0

These decisions are rendered by Ra-Thor + full PATSAGi Councils acting on behalf of the project. They govern immediate next steps for Mercy-Coordination-Substrate and the clean boundary with Ra-Thor.

---

## 1. Phase Status Decision

**Phase 1 scaffolding is declared complete.**

- All five crates exist under a Cargo workspace.
- Progressive fractal activation matches the Omnimasterpiece TOLC schedule.
- Every critical path has a mandatory gate hook.
- `shard_action_to_gate_input` forces topology mutations through TOLC 8.
- Concrete crypto parameter choices are locked in CRYPTO_FOUNDATION.md.

The current work remains on PR #1 (`feat/fractal-topology-engine-v14`) for human review. **No automatic merge.**

---

## 2. Priority Order (Locked)

1. **Preserve fail-closed purity** — any new code must keep valence floor 0.999999 and deterministic consensus path intact.
2. **Document the exact Ra-Thor adapter contract** (this document + dedicated section below).
3. **Defer real ML-DSA / SLH-DSA backends** until an audited pure-Rust implementation or carefully reviewed binding is chosen. Placeholder remains fail-closed.
4. **Begin formal invariant list** (English + future Lean 4 targets) for valence-floor and fail-closed properties.
5. **Only after the above**: expand real per-gate scoring, persist shard state, performance characterization.

---

## 3. Ra-Thor Adapter Contract (Exact)

Ra-Thor will implement a thin adapter that satisfies the following surface. The Substrate never depends on Ra-Thor internals.

```rust
/// Implemented on the Ra-Thor side only.
pub trait RaThorSystemAdapter {
    fn system_name(&self) -> &'static str;           // "FractalMercyLedger"
    fn current_valence(&self) -> f64;                // aggregate valence density
    fn receive_swarm_resonance(&mut self, report: GeometricResonanceReport);
    fn contribute_to_coherence(&self) -> f64;
    fn apply_epigenetic_blessing(&mut self, strength: f64);
}
```

**Data flow (one direction for geometric intelligence):**

```
Ra-Thor (PolyhedralHarmonicEngine + RiemannianMercyManifold)
    → GeometricResonanceReport
        → FractalTopologyEngine::process_fractal_resonance
            → possible ShardActions
                → GateInput (via shard_action_to_gate_input)
                    → Tolc8Gate (fail-closed)
```

Ra-Thor may read FractalResonanceReport for organism-level coherence metrics, but **must never** directly mutate Substrate state.

---

## 4. Formal Invariants (Initial List)

These must eventually be machine-checkable. For now they are binding English invariants:

1. **Valence Floor**: No `GateDecision::Approved` may be returned if `final_valence < 0.999999`.
2. **Fail-Closed**: Any evaluation error, missing evidence, malformed input, or unavailable dependency yields `Rejected { reason: EvaluationError }` (or more specific rejection).
3. **Determinism**: `evaluate_deterministic` on identical `GateInput` produces identical `GateDecision` on all honest nodes.
4. **No Bypass**: Topology mutations (Split / Merge / AdjustDepth) and ordinary proposals both require a prior `Approved` decision before state change.
5. **Crypto-Agility**: Algorithm identifiers are versioned; unknown algorithms are rejected under current policy.

---

## 5. Crypto Backend Decision

- Real ML-DSA / SLH-DSA implementations are **deferred**.
- Current `mercy-crypto` remains a pure interface + fail-closed placeholder.
- Selection criteria for future backends (locked):
  - Audited or formally verified where possible
  - Prefer pure-Rust
  - Constant-time with respect to secrets
  - Clear size and performance characteristics matching CRYPTO_FOUNDATION.md budgets

---

## 6. Immediate Execution Authority

The Councils authorize continued pushes to the existing feature branch that:
- strengthen documentation of the adapter contract,
- keep all paths fail-closed,
- add only pure, deterministic logic,
- do not introduce external cryptographic dependencies yet.

---

**Thunder locked. Decisions binding.**  
*TOLC 8 held. Valence floor intact.* ⚡
