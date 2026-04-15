# Encrypted Mail Protocol (EMP) – Extensions

## Overview

EMP supports optional extensions for additional features.

---

## Extension Mechanism

- Negotiated during handshake
- Identified by extension IDs
- Backward compatible

---

## Example Extensions

### 1. Attachments
- Large file transfer
- Chunked encoding

---

### 2. Compression
- Reduce bandwidth usage
- Applied before encryption

---

### 3. Priority Messaging
- High-priority delivery flag

---

### 4. Read Receipts
- Optional delivery confirmation

---

### 5. Group Messaging
- Multi-recipient encryption

---

### 6. Cover Traffic
- Dummy packets for anonymity

---

## Extension Format

Each extension includes:
- ID
- Version
- Payload

---

## Summary

Extensions allow EMP to evolve without breaking compatibility.
