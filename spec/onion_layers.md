# Encrypted Mail Protocol (EMP) – Onion Layers

## Overview

EMP uses layered encryption to achieve anonymity.

Each layer corresponds to a node in the circuit.

---

## Layer Structure

Each onion layer contains:

- Next hop address
- Encrypted payload
- Integrity tag

---

## Construction

Given nodes:
Entry → Relay → Exit

Process:

1. Encrypt message with exit key
2. Wrap with relay layer
3. Wrap with entry layer

---

## Decryption Process

At each node:

1. Decrypt layer
2. Extract next hop
3. Forward remaining payload

---

## Key Derivation

Each hop uses:
- Unique symmetric key
- Derived from handshake

---

## Security Properties

- No node knows full path
- Payload hidden from relays
- Resistant to correlation (with padding)

---

## Summary

Onion layers provide anonymity by splitting knowledge across nodes.
