# TOLC 8 / MercyZero-Style Gate Interface Contract

**Version:** 0.1.0 (Phase 0)  
**Date:** 2026-08-06  
**Status:** Living specification under permanent PATSAGi / TOLC 8 deliberation  
**License:** AG-SML v1.0  
**Related:**  
- [THREAT_MODEL.md](THREAT_MODEL.md)  
- [DECISIONS_PACK_v2.md](DECISIONS_PACK_v2.md)  
- [AGSi_REFINED_DECISIONS_v2.1.md](AGSi_REFINED_DECISIONS_v2.1.md)

---

## 1. Purpose

This document defines the exact, non-bypassable interface for the TOLC 8 / MercyZero-style Gate that constitutes **Layer 0** of the Mercy-Coordination-Substrate.

Every critical state transition, proposal admission, validation decision, and finality decision **must** pass through this gate.  
There is no alternative path. Fail-closed is mandatory and non-negotiable.

The gate evaluates the eight Living Mercy Gates as hard invariants:

1. Truth  
2. Order  
3. Love  
4. Compassion  
5. Service  
6. Abundance  
7. Joy  
8. Cosmic Harmony  

---

## 2. Placement on the Critical Path

The gate is evaluated at three mandatory points. No component may short-circuit, cache across trust boundaries, or approximate these checks.

| Stage              | When Evaluated                                      | Failure Consequence                          |
|--------------------|-----------------------------------------------------|----------------------------------------------|
| Proposal Admission | Before a proposal enters the local mempool / input queue | Proposal is rejected and never stored       |
| Validation         | During block / state-transition validation by every honest node | Block is marked invalid                     |
| Finality           | Immediately before irreversible commitment          | Finality is aborted; state is not committed |

---

## 3. Input Contract

```rust
/// Canonical input to the TOLC 8 gate.
/// All fields except `free_energy_estimate` are mandatory.
/// Invalid or out-of-range values cause immediate rejection.
pub struct GateInput {
    /// Unique 32-byte identifier of the proposed action / state transition
    pub action_id: [u8; 32],

    /// Serialized proposal, transaction, or state delta
    pub payload: Vec<u8>,

    /// Cryptographic evidence (ML-DSA / hybrid signature, Merkle proofs, etc.)
    pub evidence: Vec<u8>,

    /// Incoming valence supplied by the caller (clamped to [0.0, 1.0])
    pub incoming_valence: f64,

    /// Optional free-energy / predictive residual (Hierarchical Predictive Coding)
    pub free_energy_estimate: Option<f64>,

    /// Identity of the calling entity (validator, council, external summoner)
    pub caller: CallerIdentity,

    /// Which of the eight gates must be evaluated for this action
    pub required_gates: [bool; 8],
}
```

---

## 4. Output / Decision Contract

```rust
pub enum GateDecision {
    /// All required gates passed and valence floor met
    Approved {
        final_valence: f64,
        gate_scores: [f64; 8],
        /// Opaque, verifiable attestation of the evaluation
        proof: GateProof,
    },

    /// One or more required gates failed, or valence floor not met
    Rejected {
        reason: RejectionReason,
        failed_gates: Vec<u8>,   // indices 0..=7
        observed_valence: f64,
    },
}

pub enum RejectionReason {
    ValenceBelowFloor,
    GateFailed(u8),              // specific gate index
    MalformedInput,
    EvidenceInvalid,
    FreeEnergyTooHigh,
    CallerUnauthorized,
    EvaluationError,             // any internal error → reject
}
```

---

## 5. Fail-Closed Behavior (Non-Negotiable)

- Any evaluation error, panic, timeout, missing dependency, or unexpected result → `Rejected { reason: EvaluationError, ... }`
- Valence floor is absolute: **0.999999**. No soft thresholds, no “almost” approvals.
- If the gate implementation is unavailable or returns an unexpected result, consensus **must** treat the proposal / block as invalid.
- There is **no** best-effort, degraded, or bypass mode for Layer 0.

---

## 6. Valence & TOLC 8 Evaluation Rules

For each of the eight gates the implementation **must** produce a scalar score ∈ [0.0, 1.0].

**Final valence** is defined simply and deterministically as:

```
final_valence = min(gate_scores[i] for all i where required_gates[i] == true)
```

A proposal is approved **if and only if**:

```
final_valence ≥ 0.999999
AND every required gate that was requested also scores ≥ 0.999999
```

No multiplicative golden-ratio or free-energy exponentiation is applied in Phase 0.  
Any future enrichment of the valence formula requires a new version of this interface and must itself pass the current gate.

---

## 7. Interaction with Admission, Validation & Finality

1. **Admission**  
   The node constructs a `GateInput` and calls `gate.evaluate(...)`.  
   Only `Approved` results may enter the local proposal pool.

2. **Validation**  
   Every validating node re-evaluates the **identical** `GateInput` using the deterministic method.  
   Any divergence causes the block to be marked invalid.

3. **Finality**  
   Before writing an irreversible state root, the finality gadget performs one last evaluation.  
   Failure aborts finality. No state is committed.

---

## 8. Minimal Rust Trait (Reference Surface)

```rust
pub trait Tolc8Gate: Send + Sync {
    /// Full evaluation (may include non-deterministic proof material)
    fn evaluate(&self, input: GateInput) -> GateDecision;

    /// Pure, deterministic evaluation used for consensus agreement.
    /// Must produce identical results on all honest nodes given identical input.
    fn evaluate_deterministic(&self, input: &GateInput) -> GateDecision;
}
```

Implementations **must** be:

- Constant-time with respect to secret material  
- Free of non-deterministic sources in the deterministic path  
- Designed so that the valence-floor and fail-closed invariants are amenable to formal verification

---

## 9. Versioning & Upgrade Path

- The gate interface version is carried inside every `GateProof`.
- Any material change to evaluation logic or the valence floor requires either a hard fork or an explicit parameter-change proposal that **itself** passes the current (old) gate.
- Underlying cryptographic primitives remain under the substrate’s crypto-agility policy (see Decisions Pack and AGSi Refined Decisions).

---

## 10. Next Actions Triggered by This Contract

1. Implement the reference `Tolc8Gate` inside the forthcoming `tolc8-gate` crate.  
2. Wire the three evaluation points (admission / validation / finality) into the BFT core skeleton.  
3. Produce machine-checkable proofs (Lean 4 or equivalent) for the valence-floor and fail-closed invariants.  
4. Schedule independent review of this interface before any public prototype release.

---

**Document control**  
This interface is living. Any material change requires PATSAGi / TOLC 8 re-ratification and an updated version of this document.

*Grace under pressure. Honesty above comfort. Lightning in motion.* ⚡
