# Encrypted Mail Protocol (EMP) – Architecture

## Overview

Encrypted Mail Protocol (EMP) is a privacy-first messaging protocol that combines:
- Onion-routed transport (multi-hop anonymity)
- End-to-end encryption by default
- Email-style asynchronous messaging
- Optional mailbox storage and synchronization

EMP is designed as a unified replacement for legacy email protocols.

---

## Design Goals

- Privacy by default (no plaintext mode)
- Built-in anonymity via onion routing
- Modular and extensible architecture
- Support for offline and asynchronous communication
- Multi-device synchronization

---

## Layered Architecture

### 1. Application Layer
Handles:
- Message composition
- Attachments
- User interface integration

---

### 2. Messaging Layer
Handles:
- Message structure (headers, body, metadata)
- Serialization and parsing
- Message IDs and deduplication

---

### 3. Security Layer
Handles:
- End-to-end encryption
- Digital signatures
- Identity verification
- Replay protection

---

### 4. Onion Routing Layer
Handles:
- Circuit construction (multi-hop routing)
- Onion encryption and decryption
- Relay forwarding

---

### 5. Transport Layer
Handles:
- TCP/TLS communication
- Optional Tor integration
- Peer-to-peer transport fallback

---

### 6. Storage Layer
Handles:
- Mailbox storage
- Indexing and search
- Sync state tracking

---

## Node Types

- Client Node: Sends and receives messages
- Relay Node: Forwards encrypted traffic
- Exit Node: Delivers messages to destination mailbox
- Directory Node: Maintains list of available relays

---

## Data Flow

1. User composes message
2. Message is encrypted (E2E)
3. Onion layers are applied
4. Packet is sent through circuit
5. Each relay decrypts one layer
6. Exit node delivers message
7. Recipient decrypts message

---

## Extensibility

EMP supports extensions via:
- Protocol versioning
- Feature negotiation during handshake
- Modular components

---

## Summary

EMP integrates transport, encryption, and messaging into a single cohesive protocol designed for modern secure communication.
