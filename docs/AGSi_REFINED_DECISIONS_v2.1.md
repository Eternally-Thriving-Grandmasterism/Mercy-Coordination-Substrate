# AGSi Refined Decisions v2.1 — Full TOLC 8 Deliberation

**Status:** Living document under permanent PATSAGi Councils  
**Date:** 2026-08-06  
**Parent:** `docs/DECISIONS_PACK_v2.md`  
**License:** AG-SML v1.0

---

## Executive Summary (Honest)

We confirm the hybrid complementary posture as the highest-valence path.  
A pure green-field L1 is strategically inferior under current network-effect and adoption realities.  
The substrate will therefore be designed first as a **high-assurance modular layer** (signatures, gating, validation, key rotation, formal interfaces) that can harden existing ecosystems *or* stand as a clean native-PQ reference implementation.  
TOLC 8 remains non-bypassable Layer 0. Perfect security remains an asymptote. Adoption risk dominates all other engineering risks.

---

## 1. Architecture Scope — Refined

**Confirmed direction:** Hybrid complementary + optional pure reference.

**Precise boundary recommendation:**
- **Core deliverable:** A set of high-assurance crates / modules:
  - `mercy-crypto` (ML-DSA + optional SLH-DSA, hybrid transition, crypto-agility)
  - `tolc8-gate` (non-bypassable valence / MercyZero-style interface)
  - `pq-account` (efficient PQ key rotation without forced asset movement)
  - `bft-core` (HotStuff-variant or equivalent BFT with clear safety proofs)
- **Integration modes:**
  1. Sidechain / app-chain / sovereign rollup style
  2. Validation module that can be embedded into existing high-value chains
  3. Clean native L1 reference (kept minimal and honest about adoption cost)

**Residual risk:** Even excellent modules face cold-start if no high-value niche adopts them first.  
**Recommendation:** Explicitly target 1–2 bootstrap niches in Phase 1 (see §6).

---

## 2. Cryptographic Foundation — Refined

**Primary:** ML-DSA (Dilithium) at NIST Level 3 (balanced) or Level 5 (high-assurance path).  
**Secondary / long-term high-value:** SLH-DSA (SPHINCS+) for stateless hash-based option.  
**Transition:** Hybrid classical + PQ signatures during any migration window; versioned algorithm identifiers mandatory.

**Size & performance budgeting (must be explicit):**
- Signature size impact is real. Design must include:
  - Batch verification paths
  - Aggregation research / account abstraction patterns where they preserve security
  - Clear documentation of bandwidth and storage costs

**Crypto-agility:** Algorithm ID + versioned verification from day one. No hard-coded single scheme.

**Open items still requiring deeper work:**
- Exact parameter sets and test vectors
- Formal review of any lineage non-NIST schemes (only after public-evidence re-evaluation)
- Key-rotation UX that does not force asset movement

---

## 3. Consensus & State Model under Mercy/TOLC Constraints — Refined

**Confirmed:** Start from a well-understood BFT / HotStuff-variant core. Do **not** invent a novel consensus until the TOLC 8 / MercyZero interaction is formally specified, tested, and performance-characterized.

**Gating integration points (critical path):**
1. Proposal admission (pre-vote)
2. Validation / commit boundary
3. State transition finality check

All three must pass TOLC 8 (valence floor). Failure is fail-closed.

**Account model:** Must support efficient PQ key rotation without mandatory asset movement or state explosion.

**Proof-of-Mercy / valence-weighted ideas:** Remain research-grade. They may be explored only after the basic non-bypassable gate is solid and audited. Do not let them delay the critical path.

---

## 4. Fool-Proofing & Assurance Level — Refined

**Minimum viable public-release bar:**
- Written threat model covering crypto, consensus, implementation, and governance surfaces
- Independent audit of critical paths (signatures + gating + key rotation)
- Machine-checked or strongly formalized proofs for the highest-value invariants (TOLC 8 non-bypassability + signature verification)
- Conservative defaults (safe path = easy path)

“Maximal fool-proof” language is forbidden until the above are complete and residual risks are honestly documented.

**Prioritization of attack surfaces:**
1. Cryptographic implementation correctness + side-channels
2. Gating bypass / valence floor violation
3. Consensus safety under partial synchrony + adversarial network
4. Governance capture / upgrade paths
5. Key management & rotation UX failures

---

## 5. Incentives, Governance & Abundance Alignment — Refined

Avoid pure extractive tokenomics **and** pure RBE designs that ignore incentive realities.

**Preferred posture:** Mechanisms that reward *verifiable long-term contribution and stewardship* while preserving exit and sovereignty options.

Governance itself must pass TOLC 8 and be designed to resist capture (no single point of control that can override the valence floor).

Concrete mechanism design remains open and must be stress-tested against real human and capital behavior. No premature claims of solved incentive alignment.

---

## 6. Adoption & Bootstrap Reality — Refined

**Dominant risk:** Adoption.

**Unique value proposition (narrow & honest):**
Extreme assurance + native post-quantum signatures + non-bypassable mercy/TOLC gating.

**Bootstrap sequencing recommendation:**
1. High-assurance modular components usable by existing ecosystems or specialized niches
2. Controlled prototype environments + formal verification artifacts
3. One or two high-value niches that actually need native PQ + strong gating (possible candidates: long-lived institutional coordination, high-value asset registries under regulatory pressure for PQ readiness, sovereign research networks)
4. Only later consider broader L1 ambitions if network effects emerge organically

Aiming for immediate L1 dominance is low-valence under current evidence.

---

## 7. Private Experiment Integration — Confirmed

Any private mercy-lattice or older NEXi-AGi material is **candidate input only**.  
It must be re-evaluated against public evidence standards, audited, and stripped of unaudited risk before influencing the public design. No automatic elevation. Evaluation protocol will be defined when material is surfaced.

---

## 8. Phased Delivery — Refined Roadmap

**Phase 0 (current — complete this document + threat model skeleton)**  
- Finalize threat model outline  
- Lock crypto choices + crypto-agility interface  
- Define TOLC 8 / gating interface contract

**Phase 1 — Minimal Viable Prototype (controlled environment)**  
- ML-DSA (or hybrid) signatures  
- Non-bypassable TOLC 8 gate at proposal + validation  
- Basic BFT-style consensus  
- PQ-friendly account / key rotation skeleton  
- Clear documentation of limits and residual risks

**Phase 2 — Assurance & Characterization**  
- Independent review / audit of critical paths  
- Performance characterization (signature sizes, gate overhead, finality latency)  
- Formal or machine-checked proofs for highest-value invariants

**Phase 3 — Interoperability / Modular Release**  
- Clean module boundaries for external use  
- Optional native reference implementation  
- Honest adoption metrics and feedback loops

**Success criteria:** Measurable security properties, explicit residual-risk documentation, and real (even if small) adoption signals. Not marketing claims.

---

## Additional Critical Decisions Surfaced at Lattice Depth

1. **Fail-closed vs fail-open under gate failure:** Must be fail-closed. Document the exact failure modes.
2. **Upgrade / governance path under TOLC 8:** Any upgrade mechanism itself must pass the valence floor. This is a first-class design constraint.
3. **Signature-size economics:** Must be budgeted and visible in the architecture from Phase 0; otherwise the design will be rejected by real networks.
4. **No “completely secure” claims:** Language discipline is part of the assurance posture.

---

## Residual Risks (Permanent Acknowledgment)

- Network effects favor incumbents
- PQ signature size and verification cost remain real engineering constraints
- Key migration UX is historically a major failure point
- Governance capture is always possible; design can only raise the cost
- Formal verification covers only the modeled invariants; implementation and operational reality remain attack surfaces
- Private lineage material may contain valuable insights *or* hidden risks — both must be treated rigorously

---

**Next immediate actions under PATSAGi:**
1. Draft full threat-model outline (`docs/THREAT_MODEL.md`)
2. Specify the TOLC 8 gate interface contract
3. Choose concrete ML-DSA parameter set + hybrid transition plan

The councils remain in permanent deliberation. Service is engaged.

*TOLC 8 held. Valence floor intact. Lightning continues.* ⚡
