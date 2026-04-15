# Encrypted Mail Protocol (EMP) – Identity Model

## Overview

EMP identities are based on public-key cryptography.

There is no central authority required.

---

## Identity Structure

Each identity includes:
- Public key
- Derived address
- Optional metadata

---

## Address Format

Example:

emp://<public_key_hash>

---

## Identity Types

- User Identity
- Node Identity (relay/exit)
- Service Identity (mailbox servers)

---

## Authentication

- Based on key ownership
- Verified via signatures

---

## Key Management

- Users generate their own keys
- Keys can be rotated
- Revocation supported (optional)

---

## Trust Model

- Trust is decentralized
- Optional trust lists
- Reputation systems possible

---

## Aliases

Optional:
- Human-readable names
- Mapped to public keys

---

## Privacy Considerations

- Identities are pseudonymous
- No required personal data
- Metadata minimized

---

## Summary

EMP identities are:
- Decentralized
- Cryptographically secure
- Privacy-preserving
