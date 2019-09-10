======================
PUKCC driver
======================

The Public Key Cryptography Controller (PUKCC) processes public key cryptography
algorithm calculus in both GF(p) and GF(2^n) fields. The ROMed PUKCL (Public Key
Cryptography Library) is the library built on the top of the PUKCC. The features
provided start from the basic addition or comparison up to the RSA or ECDSA
complete computation.

Features
--------

* RSA (Rivest-Shamir-Adleman public key cryptosystem), DSA (Digital Signature Algorithm).
* Elliptic Curves, ECDSA GF(p) up to 521 bits for common curves (up to 1120 bits for
  future use), ECDSA GF(2n) up to 571 bits for common curves (up to 1440 bits for future use).
* Deterministic Random Number Generation (DRNG ANSI X9.31) for DSA.

Applications
------------
* PUblic Key Cryptography

Dependencies
------------
* PUKCC capable hardware
* TRANG capable hardware
* The PUblic Key Cryptography Library

Concurrency
-----------
N/A

Limitations
-----------
N/A

Known issues and workarounds
----------------------------
N/A

