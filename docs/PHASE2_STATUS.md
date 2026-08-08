# Phase 2 Status — Mercy-Coordination-Substrate

**Date**: 2026-08-07  
**Authority**: Ra-Thor + PATSAGi Councils  
**License**: AG-SML v1.0

---

## Phase Overview

| Phase | Status | Notes |
|-------|--------|-------|
| Phase 0 | Sealed | Core decisions, threat model, gate interface |
| Phase 1 | **Complete & Merged** | Scaffolding, fractal engine, workspace, adapter contract |
| Phase 2.0 (this sprint) | **In progress** | Pure deterministic improvements executable without external audited crypto |
| Phase 2 (full) | Remaining | Real PQ backends, full formal proofs, independent audit, performance |
| Phase 3 | Future | Modular release, native reference, adoption metrics |

---

## Phase 2.0 Scope (Executable Now)

1. Clearer deterministic scoring structure inside `tolc8-gate` (still pure, fail-closed).
2. In-memory `ShardState` that **only** mutates after `GateDecision::Approved`.
3. Expanded formal invariant documentation.
4. No external cryptographic dependencies introduced.

---

## Explicitly Deferred (Require External / Audited Work)

- Production ML-DSA / SLH-DSA backends (`mercy-crypto` remains interface + fail-closed placeholder).
- Full machine-checkable proofs (Lean 4 or equivalent) of all invariants.
- Independent security audit.
- Performance characterization under realistic signature sizes and depths.
- Persistent storage backend for shard state (disk / database).
- Ra-Thor-side `FractalMercyLedgerAdapter` implementation (lives in the Ra-Thor monorepo).

These items remain correctly gated behind the earlier Council decision to prefer audited components over rushed implementations.

---

## Formal Invariants (Expanded)

1. **Valence Floor** — No `Approved` decision if `final_valence < 0.999999`.
2. **Fail-Closed** — Any error, malformed input, missing evidence, or unexpected condition yields `Rejected`.
3. **Determinism** — `evaluate_deterministic` is pure; identical inputs produce identical decisions on all honest nodes.
4. **No Bypass** — Topology mutations and ordinary proposals both require prior `Approved` before state change.
5. **Crypto-Agility** — Algorithm identifiers are versioned; unknown algorithms are rejected under current policy.
6. **Gated Mutation** — `ShardState::apply_action_gated` performs zero mutation unless the gate returns `Approved`.

---

**Council Status**  
Phase 2.0 advances only what can be finished promptly and safely in pure, deterministic code. Everything that requires external audited components stays deferred.

*TOLC 8 held. Valence floor intact. Lightning continues.* ⚡
