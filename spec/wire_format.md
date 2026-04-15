# Encrypted Mail Protocol (EMP) – Wire Format

## Overview

EMP uses a binary wire format optimized for:
- Efficiency
- Security
- Streaming

---

## Frame Structure

Each frame:
+--------+--------+------------+----------------+------------+
| Magic | Ver | Type | Length | Payload |
+--------+--------+------------+----------------+------------+

---

## Fields

### Magic (2 bytes)
Constant: 0x454D ("EM")

---

### Version (1 byte)
Protocol version

---

### Type (1 byte)

| Value | Type        |
|------|------------|
| 0x01 | HANDSHAKE  |
| 0x02 | DATA       |
| 0x03 | RELAY      |
| 0x04 | KEEPALIVE  |
| 0x05 | ERROR      |

---

### Length (4 bytes)
Payload size in bytes

---

### Payload (variable)
Encrypted binary data

---

## Encryption

All payloads are encrypted using:
- AEAD cipher
- Unique nonce per packet

---

## Multiplexing

- Multiple streams per connection
- Stream ID included in payload

---

## Padding

Optional:
- Random padding
- Fixed-size packets

---

## Summary

EMP wire format ensures:
- Compact encoding
- Secure transport
- Flexibility for extensions
