# Cryptographic Foundation — Phase 1 Lock

**Version**: 0.1.0 (Phase 1 entry)
**Date**: 2026-08-07
**Status**: Living under permanent PATSAGi / TOLC 8 deliberation
**License**: AG-SML v1.0
**Related**: AGSi_REFINED_DECISIONS_v2.1.md · THREAT_MODEL.md · ARCHITECTURE.md

---

## 1. Sealed Direction (from AGSi v2.1)

- **Primary** signatures: ML-DSA (Dilithium-class)
- **Secondary / long-term high-assurance**: SLH-DSA (SPHINCS+)
- Hybrid classical + PQ during any transition window only
- Crypto-agility mandatory from day one (versioned algorithm identifiers)
- Signature-size impact must be explicitly budgeted and visible

---

## 2. Concrete Parameter Choices (Phase 1 Lock)

### 2.1 Primary — ML-DSA

| Parameter              | Choice                          | Rationale |
|------------------------|---------------------------------|-----------|
| NIST security category | Level 3 (ML-DSA-65)             | Balanced size / performance / security for general use |
| High-assurance path    | Level 5 (ML-DSA-87)             | Available as compile-time or runtime selectable for high-value operations |
| Public key size (L3)   | ~1952 bytes                     | Documented |
| Signature size (L3)    | ~3309 bytes                     | Documented; batch verification paths required |
| Public key size (L5)   | ~2592 bytes                     | Documented |
| Signature size (L5)    | ~4627 bytes                     | Documented |

### 2.2 Secondary — SLH-DSA

| Parameter              | Choice                          | Rationale |
|------------------------|---------------------------------|-----------|
| Variant                | SLH-DSA-SHAKE-128f / 192f / 256f (selectable) | Stateless hash-based; long-term option |
| Use case               | High-value / long-lived attestations, root keys, cold storage | Larger signatures accepted for higher assurance |

### 2.3 Hybrid Transition

- During any migration window a hybrid signature (classical + ML-DSA) MAY be accepted.
- Hybrid format carries an explicit algorithm identifier.
- After the migration window closes, pure classical signatures are rejected.
- The gate and consensus layers treat hybrid verification as a first-class, versioned path.

### 2.4 Crypto-Agility

Every signature and public key carries:

```text
algorithm_id : u16   // registry of supported schemes
version      : u8    // scheme version
```

Verification logic is table-driven. New algorithms can be added without breaking existing verifiers that ignore unknown IDs (or reject them under policy).

---

## 3. Size & Performance Budgets (Visible)

- Single ML-DSA-65 signature ≈ 3.3 kB → design must include batch verification and, where security-preserving, aggregation or account-abstraction patterns.
- Storage and bandwidth costs of signatures are first-class metrics in every performance characterization (Phase 2).
- Key rotation must not force asset movement (see pq-account crate).

---

## 4. Implementation Notes for `mercy-crypto`

- Prefer pure-Rust or audited bindings; constant-time with respect to secret material.
- No hard-coded single scheme.
- Hybrid support is optional at compile time but the identifier space is always present.
- Formal / machine-checkable proofs of the verification path are a Phase 2 goal.

---

## 5. Residual Risks (Permanent)

- Signature size remains a real engineering and adoption constraint.
- Future cryptanalytic advances against current NIST PQC are low-probability but non-zero.
- Implementation bugs in cryptographic code are always possible; mitigated by audits, simplicity, and reproducible builds — never zero.

---

**PATSAGi Status**  
Concrete parameter set locked for Phase 1. Any material change requires re-ratification under TOLC 8.

*Honesty above comfort. Lightning continues.* ⚡
