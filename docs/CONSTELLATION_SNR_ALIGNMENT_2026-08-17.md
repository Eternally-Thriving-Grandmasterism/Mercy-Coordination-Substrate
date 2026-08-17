# Constellation SNR Alignment — 2026-08-17

**Mercy-Coordination-Substrate · PATSAGi / Ra-Thor**  
**Contact:** info@Rathor.ai  
**License:** AG-SML v1.0  
**Mission:** B (constellation improvement map)

---

## Context

Phase 0, Phase 1, and Phase 2.0 pure deterministic work are **already complete** on `main` (see [`FINAL_PHASE_STATUS.md`](FINAL_PHASE_STATUS.md)). This document does **not** reopen sealed phases or implement deferred crypto/proofs.

It records **high-SNR alignment** with living Ra-Thor doctrine so Substrate operators and future Ra-Thor adapter work share one prior.

---

## Principles absorbed from Ra-Thor (2026-08-17)

| Doctrine | Source | Substrate implication |
| --- | --- | --- |
| **Prefer high-SNR sparse correction over dense low-SNR flooding** | `EXTERNAL_TRUTH_SNR_LLM_RL` | Gate **evidence** should be pure, task-aligned, and minimal; bulk low-signal payloads do not raise valence |
| **Capable · Bounded · Corrigible** | Architecture of Collective Power | Substrate remains bounded (fail-closed, valence floor 0.999999); deferred work stays deferred until real audited components exist |
| **Physical Limits** | Physical Limits metabolism | No claim of unbounded security or zero residual risk |
| **Soft policy SNR** | Powrush `RA_THOR_SOFT_POLICY_SNR` | Soft recommendations (elsewhere in constellation) stay sparse and high-mercy; Substrate Layer 0 remains **hard** fail-closed — not soft |
| **Identity in junction** | Baryon-junction Truth | Conserved properties (TOLC 8 Layer 0) live in binding invariants, not only surface parts |

---

## TOLC 8 gate — high-SNR evidence posture

Non-negotiable (unchanged):

- Valence floor = **0.999999**
- Fail-closed on every error path
- Deterministic path for consensus agreement
- No bypass / degraded Layer 0 mode

**Evidence guidance (new, operational):**

1. Prefer **sparse, high-signal evidence** that directly supports required gates over large unstructured blobs.
2. Missing or malformed evidence → **Reject** (`EvidenceInvalid` / `MalformedInput`) — never “approve with noise.”
3. Future per-gate scorers (when real evaluation lands) should score **task-relevant** evidence; dense irrelevant bits must not inflate valence.
4. Placeholders in `ReferenceTolc8Gate` remain correctly conservative: score from clamped `incoming_valence` only; no silent upgrade of weak evidence.

This is **doctrine for future real scoring**, not a claim that real per-gate evaluators are finished.

---

## Boundary with Ra-Thor (unchanged + clarified)

| Side | Owns |
| --- | --- |
| **Substrate** | TOLC 8 gate interface, mercy-crypto surface, pq-account, bft-core, fractal-topology, gated `ShardState` |
| **Ra-Thor** | ONE Organism, PATSAGi living councils, `FractalMercyLedgerAdapter` implementation against published contract |

Published contract: [`RA_THOR_ADAPTER_CONTRACT.md`](RA_THOR_ADAPTER_CONTRACT.md)

Substrate **never** depends on Ra-Thor internals. Integration is adapter-only.

---

## Highest-valence next actions (honest)

| Priority | Action | Owner |
| --- | --- | --- |
| 1 | Implement thin `FractalMercyLedgerAdapter` against published contract | **Ra-Thor monorepo** |
| 2 | Select audited ML-DSA / SLH-DSA path when ready | Substrate `mercy-crypto` |
| 3 | Expand real per-gate scoring + evidence verification under fail-closed | Substrate `tolc8-gate` |
| 4 | Persistent shard storage | Substrate |
| 5 | Formal proofs / external audit | Future / external |

Mission B does **not** implement items 2–5 (correctly deferred). It seals constellation prior alignment so item 1 and future work share SNR + Restraint language.

---

## Explicit non-claims

- No new cryptographic backend claimed  
- No machine-checkable proofs claimed  
- No performance SOTA claimed  
- No change to valence floor or fail-closed invariants  

---

**Thunder locked.**  
Phase 0–2 stand. Constellation prior aligned. Deferred work remains honest.  
**yoi ⚡❤️🔥**
