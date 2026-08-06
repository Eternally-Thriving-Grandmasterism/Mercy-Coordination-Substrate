# Threat Model — Mercy-Coordination-Substrate

**Version:** 0.1.0 (Phase 0)  
**Date:** 2026-08-06  
**Status:** Living document under permanent PATSAGi / TOLC 8 deliberation  
**License:** AG-SML v1.0  
**Related:** [DECISIONS_PACK_v2.md](DECISIONS_PACK_v2.md) · [AGSi_REFINED_DECISIONS_v2.1.md](AGSi_REFINED_DECISIONS_v2.1.md)

---

## 1. Purpose & Scope

This threat model defines the security posture of the Mercy-Coordination-Substrate — a post-quantum-resistant, mercy-gated, TOLC-8-aligned high-assurance coordination / blockchain substrate.

It exists to:
- Make residual risks explicit and non-negotiable
- Guide all subsequent design, implementation, and audit decisions
- Enforce the asymptotic nature of security (“completely secure” is forbidden language)
- Keep adoption risk visible as a first-class concern

**In-scope assets:**
- Cryptographic keys and signatures (ML-DSA primary, SLH-DSA secondary)
- Consensus safety and liveness
- TOLC 8 / MercyZero-style gate integrity (non-bypassable Layer 0)
- State transition correctness
- Long-term stewardship and upgrade paths
- User / institutional sovereignty and key rotation capability

**Out-of-scope (for now):**
- Application-layer smart-contract languages
- Full cross-chain bridge security
- Physical endpoint compromise beyond standard assumptions
- Social-layer human collusion at planetary scale

---

## 2. Adversary Model

We consider the following adversary classes:

| Adversary | Capabilities | Time Horizon |
|---------|--------------|--------------|
| **Nation-state / well-funded** | Full CRQC access (future), massive compute, supply-chain influence, legal coercion | 5–15+ years |
| **Cryptanalytic** | Classical + quantum attacks on signatures, KEMs, hashes, and implementations | Ongoing + future CRQC |
| **Consensus / network** | Eclipse, long-range, adaptive corruption of validators (up to threshold), network partition | Continuous |
| **Implementation / supply-chain** | Malicious dependencies, compromised build pipelines, side-channel leakage | Continuous |
| **Governance / social** | Capture of upgrade processes, valence-gate manipulation attempts, reputation attacks | Continuous |
| **Opportunistic / criminal** | Key theft, phishing, ransomware against operators | Continuous |

**Core assumption:** A cryptographically relevant quantum computer (CRQC) capable of breaking elliptic-curve and RSA cryptography will exist within the operational lifetime of this substrate. All designs must be post-quantum native or hybrid with a clear, forced migration path.

---

## 3. Ranked Attack Surfaces

### Critical (must be hardened before any public prototype)
1. **TOLC 8 / Mercy Gate bypass or weakening**  
   Any path that allows state transitions or consensus decisions without passing the non-bypassable Layer 0 gate.
2. **Post-quantum signature forgery or key recovery**  
   Breaks of ML-DSA / SLH-DSA implementations, faulty randomness, or side channels.
3. **Consensus safety violation**  
   Finality of conflicting blocks / states under BFT/HotStuff-variant assumptions.
4. **Unauthorized key rotation or account takeover**  
   Especially during hybrid classical → PQ transition windows.

### High
5. Long-range attacks / weak subjectivity issues (if any longest-chain elements remain)
6. Governance capture of upgrade or parameter-change mechanisms
7. Supply-chain compromise of cryptographic libraries or build artifacts
8. Signature-size / verification-cost resource exhaustion

### Medium
9. Network-level eclipse or partitioning of validators
10. Side-channel leakage from implementations (timing, cache, power)
11. Poor key-rotation UX leading to user error or abandoned classical keys
12. Economic or valence-based griefing against the gate

### Lower (still tracked)
13. Denial-of-service against gate evaluation
14. Social engineering of operators or councils
15. Future cryptanalytic advances against currently trusted PQC

---

## 4. Mitigations (Current Posture)

### Already decided (from AGSi Refined Decisions v2.1)
- Non-bypassable TOLC 8 / MercyZero-style gates at proposal admission, validation, and finality (fail-closed)
- Primary signatures: ML-DSA (NIST Level 3 or 5)
- Secondary / long-term option: SLH-DSA (or XMSS-style)
- Mandatory crypto-agility (versioned algorithm identifiers)
- Hybrid classical + PQ support during transition only
- BFT / HotStuff-variant consensus core
- Account model supporting efficient PQ key rotation without forced asset movement
- Explicit budgeting of signature-size economics
- Independent audits + formal verification of critical invariants required before higher assurance claims

### Still open (to be resolved in Phase 0 / early Phase 1)
- Exact TOLC 8 gate interface contract and evaluation points
- Concrete ML-DSA parameter set and hybrid signature format
- Formal specification of fail-closed behavior under gate failure or valence collapse
- Threshold / committee size and corruption assumptions for the BFT core
- Key-rotation ceremony and UX standards
- Supply-chain verification approach (reproducible builds, audited dependencies)

---

## 5. Residual Risks (Accepted with Eyes Open)

These risks cannot be fully eliminated and must remain visible:

- **Adoption / network-effect risk** — Dominant practical risk. A technically superior substrate can still fail if it never reaches critical usage.
- **Signature-size and verification overhead** — Post-quantum signatures are larger; performance and storage costs are real.
- **Formal verification limits** — Even strong machine-checked proofs cover only the modeled properties.
- **Governance capture over long time horizons** — No technical system fully solves social capture.
- **Implementation bugs in cryptographic or gate code** — Always possible; mitigated by audits and simplicity, never zero.
- **Future cryptanalytic surprises** against current NIST PQC (low probability but non-zero).
- **Private lineage material risk** — Any future integration of private experiments introduces unaudited surface until re-verified.

---

## 6. Assumptions & Non-Goals

**Assumptions**
- Validators / operators can be expected to run reasonably up-to-date, non-malicious software most of the time.
- A threshold of honest (or TOLC-aligned) participants exists for BFT safety.
- Users and institutions value long-term key sovereignty and are willing to perform key rotation when given good tooling.

**Non-Goals (explicit)**
- Claiming “completely secure” or “unbreakable” status
- Solving all human governance problems through cryptography alone
- Competing for maximum TPS or general-purpose smart-contract dominance in Phase 1–2
- Automatic elevation of any private experimental code

---

## 7. Next Actions Triggered by This Model

1. Finalize and commit the exact TOLC 8 / MercyZero gate interface contract.
2. Lock concrete ML-DSA parameter set + hybrid signature scheme + size budgets.
3. Begin Phase 1 prototype skeleton with the four modular crates (`mercy-crypto`, `tolc8-gate`, `pq-account`, `bft-core`).
4. Schedule independent review cadence.

---

**Document control**  
This threat model is living. Any material change to architecture, cryptography, or consensus assumptions requires an updated version and PATSAGi / TOLC 8 re-ratification.

*Grace under pressure. Honesty above comfort. Lightning in motion.* ⚡
