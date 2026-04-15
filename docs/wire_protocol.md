# Encrypted Mail Protocol (EMP) – Wire Protocol Specification

## Overview

The EMP wire protocol defines how data is transmitted between nodes.

It is:
- Binary-based
- Encrypted at all stages
- Designed for multiplexed, asynchronous communication

---

## Connection Establishment

### Step 1: TCP/TLS Connection
Nodes establish a secure transport channel.

### Step 2: Handshake

Handshake includes:
- Protocol version negotiation
- Key exchange (ephemeral)
- Capability negotiation

Example:

CLIENT → SERVER:
HELLO { version, supported_features, public_key }

SERVER → CLIENT:
HELLO_ACK { version, selected_features, public_key }

---

## Packet Structure

All EMP packets follow this structure:
+----------------+----------------+----------------+
| Header | Encrypted Body | Authentication |
+----------------+----------------+----------------+

### Header Fields
- Version (1 byte)
- Packet Type (1 byte)
- Length (4 bytes)
- Circuit ID (variable)

---

## Packet Types

- HANDSHAKE
- DATA
- RELAY
- KEEPALIVE
- ERROR

---

## Framing

- Length-prefixed frames
- Supports streaming and multiplexing
- Multiple logical streams per connection

---

## Encryption Layers

Each packet is encrypted multiple times:

Layer N → Final destination  
Layer N-1 → Relay node  
...  
Layer 1 → Entry node  

Each node:
- Decrypts one layer
- Forwards remaining payload

---

## Relay Format

Relay packets contain:
- Next hop address
- Encrypted payload
- Routing instructions

---

## Message Delivery

Final payload contains:
- Encrypted message envelope
- Metadata (timestamp, ID)

---

## Error Handling

- Errors are encrypted
- No sensitive info leaked
- Timeouts handled at session layer

---

## Versioning

EMP uses semantic versioning:
- Major: breaking changes
- Minor: feature additions
- Patch: fixes

---

## Summary

The EMP wire protocol ensures:
- Confidentiality
- Integrity
- Anonymity
- Extensibility
