# Encrypted Mail Protocol (EMP) – Cryptographic Specification

## Overview

EMP enforces encryption at all layers:
- Transport encryption
- Onion layer encryption
- End-to-end message encryption

---

## Cryptographic Primitives

Recommended:
- Symmetric: AES-256-GCM or ChaCha20-Poly1305
- Asymmetric: X25519 / Ed25519
- Hashing: SHA-256 or BLAKE3

---

## Key Types

- Identity Key (long-term)
- Session Key (ephemeral)
- Onion Layer Keys (per hop)

---

## Handshake

Uses:
- Ephemeral Diffie-Hellman
- Forward secrecy
- Mutual authentication (optional)

---

## End-to-End Encryption

Message encryption:
- Encrypt with recipient public key
- Sign with sender private key

---

## Onion Encryption

Each hop has:
- Unique symmetric key
- Separate encryption layer

---

## Integrity

All messages include:
- Authentication tags (AEAD)
- Signature verification

---

## Replay Protection

- Unique message IDs
- Timestamps
- Nonce tracking

---

## Key Storage

- Secure local storage
- Optional hardware-backed keys

---

## Summary

EMP ensures:
- Confidentiality
- Integrity
- Forward secrecy
