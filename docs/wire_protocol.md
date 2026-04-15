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
