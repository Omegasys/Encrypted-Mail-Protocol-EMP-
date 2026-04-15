# Encrypted Mail Protocol (EMP) – High-Level Protocol Definition

## Overview

EMP is a secure, onion-routed messaging protocol designed to replace traditional email systems with:

- Built-in anonymity (onion routing)
- End-to-end encryption
- Asynchronous message delivery
- Decentralized identity

---

## Core Principles

1. **Privacy by Default**
   - All communication is encrypted
   - No plaintext fallback

2. **Anonymity**
   - Multi-hop onion routing
   - No single node knows full path

3. **Decentralization**
   - No central authority required
   - Peer-based identity

4. **Modularity**
   - Transport, crypto, and routing are separable

---

## Protocol Components

### 1. Nodes
- Client Nodes
- Relay Nodes
- Exit Nodes
- Directory Nodes

---

### 2. Circuits

A circuit is a temporary path through multiple nodes:

Client → Entry → Relay → Exit → Destination

---

### 3. Messages

Messages consist of:
- Header (metadata)
- Body (content)
- Attachments (optional)

All messages are encrypted end-to-end.

---

### 4. Sessions

Sessions are established between peers using:
- Secure handshake
- Ephemeral keys
- Capability negotiation

---

### 5. Delivery Modes

- Direct (peer-to-peer)
- Store-and-forward (mailbox)
- Hybrid

---

## Protocol Flow

1. Discover nodes
2. Build circuit
3. Establish session
4. Encrypt message
5. Wrap in onion layers
6. Send through circuit
7. Deliver to recipient

---

## Security Guarantees

- Confidentiality
- Integrity
- Forward secrecy
- Sender/receiver anonymity (within limits)

---

## Summary

EMP defines a unified system for secure, anonymous, asynchronous communication.
