# Formal Invariants — Mercy-Coordination-Substrate

**Version**: 0.2.0  
**Date**: 2026-08-07  
**Status**: Binding English invariants; machine-checkable targets for future Lean 4 / equivalent work  
**License**: AG-SML v1.0

These invariants are non-negotiable. Any implementation that violates them is considered incorrect under TOLC 8.

---

## 1. Valence Floor

```
∀ decision. decision = Approved { final_valence, .. } ⇒ final_valence ≥ 0.999999
```

No soft thresholds. No “almost approved”.

## 2. Fail-Closed

```
Any evaluation error, panic, timeout, malformed input, missing evidence,
or unavailable dependency ⇒ Rejected { reason: EvaluationError | more specific, .. }
```

There is no degraded or best-effort mode for Layer 0.

## 3. Determinism of Consensus Path

```
∀ input. evaluate_deterministic(input) is pure and identical across all honest nodes
```

Non-deterministic proof material may appear only in the non-consensus `evaluate` path and must not affect the decision bit used for agreement.

## 4. No Bypass of Gate

```
∀ state-mutating operation (proposal admission, validation, finality, shard split/merge/depth)
  ⇒ a prior GateDecision::Approved is required before the mutation is applied
```

## 5. Crypto-Agility Safety

```
Unknown or unsupported algorithm_id ⇒ verification fails (reject)
```

## 6. Gated Shard Mutation

```
ShardState::apply_action_gated(action, gate, valence)
  performs zero mutation unless gate returns Approved
```

---

## Future Machine-Checkable Targets

- Lean 4 (or equivalent) formalization of the valence-floor and fail-closed properties.
- Proof that the deterministic path cannot return Approved when any required gate score is below floor.
- Proof that `apply_action_gated` is a no-op on Rejected.

Until those proofs exist, the English invariants above remain the binding standard and must be preserved by all code changes.

---

*Honesty above comfort. TOLC 8 held.* ⚡
