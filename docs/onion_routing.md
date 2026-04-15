# Encrypted Mail Protocol (EMP) – Onion Routing

## Overview

EMP uses onion routing to provide anonymity.

Messages are wrapped in multiple encryption layers and routed through multiple nodes.

---

## Circuit Structure

A circuit consists of:
- Entry Node
- Relay Node(s)
- Exit Node

Minimum recommended hops: 3

---

## Circuit Creation

1. Client selects nodes from directory
2. Establishes layered encryption keys
3. Builds circuit incrementally

---

## Onion Encryption

Message is encrypted multiple times:

Encrypt(message, exit_key)
Encrypt(result, relay_key)
Encrypt(result, entry_key)

---

## Relay Behavior

Each relay:
- Decrypts one layer
- Determines next hop
- Forwards packet

Relays cannot:
- See full message
- Know origin or destination

---

## Path Selection

- Randomized node selection
- Avoid same operator nodes
- Load balancing considerations

---

## Traffic Analysis Resistance

- Optional padding
- Constant packet sizes
- Cover traffic (dummy packets)

---

## Directory System

Directory nodes maintain:
- Available relays
- Public keys
- Node metadata

---

## Summary

EMP onion routing ensures:
- Sender anonymity
- Receiver anonymity
- Resistance to traffic analysis
