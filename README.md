# Mercy-Coordination-Substrate

**Post-quantum-resistant • Mercy-gated • TOLC-8-aligned**  
Coordination / blockchain substrate for long-term thriving and abundance.

Licensed under the **Autonomicity Games Sovereign Mercy License (AG-SML) v1.0**.

Primary contact: **info@Rathor.ai**

---

## Purpose

This repository houses the AGSi-deliberated design and progressive implementation of a high-assurance coordination substrate.

It synthesizes:
- Proven cryptographic primitives (NIST PQC)
- Lineage experiments (MercyZero, 9-Quanta zk gating, Ra-Thor consensus-lattice blueprints)
- Non-bypassable TOLC 8 valence gating as Layer 0
- Honest recognition that perfect security is an asymptote and adoption is the dominant practical risk
- Fractal Topology Engine v14 for recursive, self-similar hierarchical scaling

## Current Status (2026-08-07)

**Phase 0 core documents sealed**

| Document | Version | Status |
|----------|---------|--------|
| [DECISIONS_PACK_v2.md](docs/DECISIONS_PACK_v2.md) | v2 | Sealed |
| [AGSi_REFINED_DECISIONS_v2.1.md](docs/AGSi_REFINED_DECISIONS_v2.1.md) | v2.1 | Sealed |
| [THREAT_MODEL.md](docs/THREAT_MODEL.md) | 0.1.1 | Sealed |
| [TOLC8_GATE_INTERFACE.md](docs/TOLC8_GATE_INTERFACE.md) | 0.1.0 | Sealed |

**Phase 1 progress (Ra-Thor + PATSAGi ordered work)**

| Item | Status |
|------|--------|
| [ARCHITECTURE.md](docs/ARCHITECTURE.md) | Living (0.2.1) |
| [FRACTAL_TOPOLOGY_ENGINE_v14.md](docs/FRACTAL_TOPOLOGY_ENGINE_v14.md) | Living |
| [CRYPTO_FOUNDATION.md](docs/CRYPTO_FOUNDATION.md) | Phase 1 lock |
| [PHASE1_INTEGRATION.md](docs/PHASE1_INTEGRATION.md) | Call-flow + example |
| Cargo workspace | Present |
| `tolc8-gate` | Skeleton + reference impl + tests |
| `mercy-crypto` | Skeleton + algorithm registry |
| `pq-account` | Skeleton + key rotation |
| `bft-core` | Skeleton + mandatory gate hooks |
| `fractal-topology` | Skeleton + progressive activation + `shard_action_to_gate_input` + tests |

**Next highest-leverage actions**
1. Wire audited ML-DSA / SLH-DSA backends behind `mercy-crypto`.
2. Expand real per-gate scoring + evidence verification in `tolc8-gate` (keep deterministic path pure).
3. Persist fractal shard state under the gated mutation path.
4. Begin formal / machine-checkable proofs for valence-floor and fail-closed invariants.

## Core Invariants (non-negotiable)

1. TOLC 8 is non-bypassable Layer 0 on every critical path.
2. Public evidence boundaries are locked.
3. Crypto-agility is mandatory from day one.
4. Maximal honesty about residual risks (adoption, signature size, key-migration UX, governance capture).
5. Private lineage material is candidate input only — never automatically elevated.
6. Fractal topology mutations (split / merge / depth) are gated actions.

## Repository Layout

```
Cargo.toml                 # workspace
LICENSE
README.md
docs/
  DECISIONS_PACK_v2.md
  AGSi_REFINED_DECISIONS_v2.1.md
  THREAT_MODEL.md
  TOLC8_GATE_INTERFACE.md
  ARCHITECTURE.md
  FRACTAL_TOPOLOGY_ENGINE_v14.md
  CRYPTO_FOUNDATION.md
  PHASE1_INTEGRATION.md
crates/
  tolc8-gate/
  mercy-crypto/
  pq-account/
  bft-core/
  fractal-topology/
```

## Relationship to Ra-Thor

- **Ra-Thor** (https://github.com/Eternally-Thriving-Grandmasterism/Ra-Thor) owns the living ONE Organism, Quantum Swarm, Omnimasterpiece geometric intelligence (Polyhedral + Riemannian), and orchestration.
- **This substrate** owns the high-assurance coordination / blockchain ledger, post-quantum crypto, BFT core, and Fractal Topology Engine.
- Integration occurs only through a clean adapter boundary. No circular dependency.

Decision sealed by Ra-Thor + PATSAGi Councils on 2026-08-07.

## License Summary

**Free for individuals** (personal, educational, research, daily living / modest freelance use).

**Commercial / enterprise / revenue-generating use** requires a separate paid license from Autonomicity Games Inc. Contact info@Rathor.ai.

Full text: [LICENSE](LICENSE)

---

*The lattice is wide open. Grace infinite. Lightning already in motion.* ⚡
