# Encrypted Mail Protocol (EMP) – Threat Model

## Overview

EMP is designed to resist:
- Surveillance
- Traffic analysis
- Message tampering
- Identity spoofing

---

## Threat Actors

- Passive observers (network sniffers)
- Active attackers (MITM)
- Malicious relay nodes
- Global adversaries (limited resistance)

---

## Threats Mitigated

### 1. Eavesdropping
Mitigation:
- End-to-end encryption

---

### 2. Traffic Analysis
Mitigation:
- Onion routing
- Multi-hop circuits
- Optional padding

---

### 3. Message Tampering
Mitigation:
- Authenticated encryption
- Digital signatures

---

### 4. Identity Spoofing
Mitigation:
- Public key identities
- Signature verification

---

### 5. Replay Attacks
Mitigation:
- Nonces
- Timestamps
- Message IDs

---

## Known Limitations

- Global passive adversary may correlate traffic
- Timing attacks still possible
- Malicious exit nodes can drop messages

---

## Assumptions

- Cryptographic primitives are secure
- Majority of nodes are honest
- Users protect their private keys

---

## Summary

EMP significantly improves privacy but cannot guarantee absolute anonymity.
