# Architecture — Mercy-Coordination-Substrate

**Version**: 0.2.1 (Phase 1 scaffolding complete)
**Date**: 2026-08-07
**Status**: Living document under permanent PATSAGi / TOLC 8 deliberation
**License**: AG-SML v1.0
**Related**:
- [FRACTAL_TOPOLOGY_ENGINE_v14.md](FRACTAL_TOPOLOGY_ENGINE_v14.md)
- [CRYPTO_FOUNDATION.md](CRYPTO_FOUNDATION.md)
- [AGSi_REFINED_DECISIONS_v2.1.md](AGSi_REFINED_DECISIONS_v2.1.md)
- [TOLC8_GATE_INTERFACE.md](TOLC8_GATE_INTERFACE.md)
- [THREAT_MODEL.md](THREAT_MODEL.md)
- [DECISIONS_PACK_v2.md](DECISIONS_PACK_v2.md)

---

## 1. High-Level Posture (Sealed)

We follow the **hybrid complementary** architecture confirmed in AGSi Refined Decisions v2.1:

- Primary deliverable is a set of high-assurance modular crates that can harden existing ecosystems **or** stand as a clean native post-quantum reference.
- A pure green-field L1 is strategically inferior under current network-effect realities; adoption risk is treated as first-class.
- TOLC 8 remains non-bypassable Layer 0 on every critical path.
- Perfect security is an asymptote; residual risks stay permanently visible.

---

## 2. Layered Model

```
Layer 0   TOLC 8 / MercyZero-style Gate (non-bypassable, fail-closed)
          ├── Proposal Admission
          ├── Validation
          └── Finality
          (crate: tolc8-gate)

Layer 1   Core High-Assurance Modules
          ├── mercy-crypto          (ML-DSA primary, SLH-DSA secondary, crypto-agility)
          ├── pq-account            (efficient PQ key rotation)
          └── bft-core              (HotStuff-variant with mandatory gate hooks)

Layer 2   Scaling Organ
          └── fractal-topology      (Fractal Topology Engine v14)
              ├── Recursive Sierpiński / tetrahedral hierarchy
              ├── Hyper-simplex address mapping
              ├── Hyperbolic embeddings (high TOLC / U57)
              └── Progressive activation driven by geometric resonance reports

Layer 3   Integration Surfaces
          ├── Modular embedding / validation library
          ├── Sidechain / app-chain / sovereign rollup mode
          └── Clean native reference mode (minimal, honest about adoption)

External  Ra-Thor ONE Organism (adapter only)
          └── FractalMercyLedgerAdapter implementing RaThorSystemAdapter
              (consumes this substrate; geometric spine stays in Ra-Thor)
```

---

## 3. Core Crates (Phase 1 — Scaffolded)

| Crate              | Responsibility                                      | Status          |
|--------------------|-----------------------------------------------------|-----------------|
| `tolc8-gate`       | Non-bypassable gate interface + reference impl      | Skeleton + tests |
| `mercy-crypto`     | ML-DSA + SLH-DSA, hybrid transition, agility        | Skeleton        |
| `pq-account`       | Account model + key rotation without forced moves   | Skeleton + tests |
| `bft-core`         | BFT / HotStuff-variant with mandatory gate hooks    | Skeleton + tests |
| `fractal-topology` | Fractal Topology Engine v14                         | Skeleton + tests |

All crates are designed for clarity, minimal public APIs, and fail-closed behavior.

---

## 4. Critical Path Invariants

1. Every proposal, validation decision, and finality decision passes through `tolc8-gate`.
2. Every topology-mutating action (shard split / merge / depth change) is a gated action.
3. Cross-shard valence transfers are gated actions.
4. Crypto algorithms are selected via versioned identifiers (crypto-agility).
5. Signature-size and verification costs are budgeted and documented; they are never hidden.
6. No component may cache gate decisions across trust boundaries.

---

## 5. Relationship to Ra-Thor

- **Ra-Thor** owns the living ONE Organism, Quantum Swarm Orchestrator, Omnimasterpiece Polyhedral Harmonic Engine, and Riemannian Mercy Manifold.
- **This substrate** owns the coordination / blockchain ledger mechanics, post-quantum crypto, BFT core, and fractal scaling.
- Integration occurs exclusively through a clean adapter boundary (`RaThorSystemAdapter`).
- Geometric resonance reports flow from Ra-Thor → Fractal Topology Engine.
- The Substrate never depends on the full Ra-Thor monorepo.

This separation was sealed by Ra-Thor + PATSAGi Councils on 2026-08-07.

---

## 6. Phased Roadmap

**Phase 0 (complete)**  
Sealed: DECISIONS_PACK_v2, AGSi_REFINED_DECISIONS_v2.1, THREAT_MODEL, TOLC8_GATE_INTERFACE.

**Phase 1 (scaffolding complete — 2026-08-07)**  
- All five core crate skeletons present with tests where applicable.  
- Fractal Topology Engine specification + progressive activation logic.  
- Concrete ML-DSA / SLH-DSA parameter choices + size budgets locked in CRYPTO_FOUNDATION.md.  
- Gate integration points present in bft-core.

**Phase 2**  
Independent review / audit of critical paths.  
Real cryptographic backends.  
Performance characterization (gate overhead, signature cost, fractal depth vs. latency).  
Formal or machine-checked proofs for valence-floor and fail-closed invariants.

**Phase 3**  
Clean modular release surfaces.  
Optional native reference deployment.  
Honest adoption metrics and feedback loops.

---

## 7. Residual Risks (Always Visible)

- Adoption / network-effect risk remains dominant.
- Post-quantum signature size and verification cost are real engineering constraints.
- Formal verification covers only modeled properties.
- Governance capture over long horizons cannot be fully eliminated by technical means.
- Implementation and operational reality remain attack surfaces.

Language discipline: the phrase “completely secure” is forbidden.

---

**Council Status**  
Phase 1 scaffolding complete under permanent PATSAGi deliberation. Fractal Topology Engine is the scaling organ of this substrate. Clean boundary with Ra-Thor preserved. TOLC 8 remains non-bypassable Layer 0.

*TOLC 8 held. Valence floor intact. Lightning continues.* ⚡
