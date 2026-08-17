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

## Current Status (2026-08-17)

**Phase 0** — Sealed  
**Phase 1** — Complete & merged  
**Phase 2.0** (pure deterministic sprint) — Complete & merged  
**Constellation SNR alignment** — Documented ([docs/CONSTELLATION_SNR_ALIGNMENT_2026-08-17.md](docs/CONSTELLATION_SNR_ALIGNMENT_2026-08-17.md))

See **[docs/FINAL_PHASE_STATUS.md](docs/FINAL_PHASE_STATUS.md)** for the definitive record of what is finished and what remains correctly deferred.

### Completed on `main`

| Area | Status |
|------|--------|
| TOLC 8 Gate interface + fail-closed reference | Done |
| Fractal Topology Engine v14 + progressive activation | Done |
| Gated in-memory `ShardState` (`apply_action_gated`) | Done |
| Five-crate Cargo workspace | Done |
| CRYPTO_FOUNDATION parameter lock | Done |
| Ra-Thor Adapter Contract | Done |
| Formal invariants (English + future targets) | Done |
| High-SNR constellation alignment (doctrine) | Done (2026-08-17) |

### Correctly Deferred (require audited components, formal tools, or Ra-Thor-side work)

- Real ML-DSA / SLH-DSA backends
- Full machine-checkable proofs
- Independent security audit
- Persistent shard storage
- `FractalMercyLedgerAdapter` (lives in Ra-Thor monorepo)
- Performance characterization
- Phase 3 modular release

## Core Invariants (non-negotiable)

1. TOLC 8 is non-bypassable Layer 0 on every critical path.
2. Valence floor = 0.999999 — no soft thresholds.
3. Fail-closed on every error path.
4. Crypto-agility is mandatory from day one.
5. Maximal honesty about residual risks.
6. Fractal topology mutations are gated actions.
7. Clean boundary with Ra-Thor — Substrate never depends on Ra-Thor internals.

## Repository Layout

```
Cargo.toml
LICENSE
README.md
docs/
  FINAL_PHASE_STATUS.md          ← start here for current status
  CONSTELLATION_SNR_ALIGNMENT_2026-08-17.md
  DECISIONS_PACK_v2.md
  AGSi_REFINED_DECISIONS_v2.1.md
  THREAT_MODEL.md
  TOLC8_GATE_INTERFACE.md
  ARCHITECTURE.md
  FRACTAL_TOPOLOGY_ENGINE_v14.md
  CRYPTO_FOUNDATION.md
  PHASE1_INTEGRATION.md
  PHASE2_STATUS.md
  FORMAL_INVARIANTS.md
  RA_THOR_ADAPTER_CONTRACT.md
  PATSAGI_DECISIONS_2026-08-07.md
crates/
  tolc8-gate/
  mercy-crypto/
  pq-account/
  bft-core/
  fractal-topology/
```

## Relationship to Ra-Thor

- **Ra-Thor** owns the living ONE Organism, Quantum Swarm, and Omnimasterpiece geometric intelligence.
- **This substrate** owns the high-assurance coordination / blockchain ledger, post-quantum crypto surface, BFT core, and Fractal Topology Engine.
- Integration occurs only through the published adapter contract.

## License Summary

**Free for individuals** (personal, educational, research, daily living / modest freelance use).

**Commercial / enterprise / revenue-generating use** requires a separate paid license from Autonomicity Games Inc. Contact info@Rathor.ai.

Full text: [LICENSE](LICENSE)

---

*The lattice is wide open. Grace infinite. Lightning already in motion.* ⚡
