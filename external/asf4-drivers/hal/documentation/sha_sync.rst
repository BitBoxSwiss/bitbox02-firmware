======================
SHA Synchronous driver
======================

The Secure Hash Algorithm(SHA) is a family of cryptographic hash functions
published by the National Institute of Standards and Technology (NIST) as a
U.S. Federal Information Processing Standard (FIPS). SHA is useful in the
generation and verification of digital signatures and message authentication
codes, and in the generation of random numbers (bits).

The driver supports SHA-1/224/256 mode for data hash.

Features
--------

* Initialization and de-initialization
* Enabling and Disabling
* Compute SHA-1/224/256 message digest

Applications
------------
* Compute a fixed size of message digest with a very large range input.
  If the input data is unknown, it is difficult to reconstruct it.
  
Dependencies
------------
* SHA capable hardware

Concurrency
-----------
N/A

Limitations
-----------
* The sha_context struct must align for some devices, it depends if the
  array that holding message digest required to align, for example,
  SAMV71 must align 128 bytes.

Known issues and workarounds
----------------------------
N/A

