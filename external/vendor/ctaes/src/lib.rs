#![no_std]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;

use core::mem::MaybeUninit;

// FFI view of the C structs.
// Layout taken directly from *ctaes.h*.

#[repr(C)]
struct AESState {
    slice: [u16; 8],
}

#[repr(C)]
struct AES128Ctx {
    rk: [AESState; 11],
}

#[repr(C)]
struct AES192Ctx {
    rk: [AESState; 13],
}

#[repr(C)]
struct AES256Ctx {
    rk: [AESState; 15],
}

#[repr(C)]
struct AES128CbcCtx {
    ctx: AES128Ctx,
    iv: [u8; 16],
}

#[repr(C)]
struct AES192CbcCtx {
    ctx: AES192Ctx,
    iv: [u8; 16],
}

#[repr(C)]
struct AES256CbcCtx {
    ctx: AES256Ctx,
    iv: [u8; 16],
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Error {
    /// The CBC plain/cipher buffer is not a multiple of 16, the block size.
    NotBlockAligned,
}

unsafe extern "C" {
    fn AES128_init(ctx: *mut AES128Ctx, key16: *const u8);
    fn AES128_encrypt(ctx: *const AES128Ctx, blocks: usize, cipher: *mut u8, plain: *const u8);
    fn AES128_decrypt(ctx: *const AES128Ctx, blocks: usize, plain: *mut u8, cipher: *const u8);

    fn AES192_init(ctx: *mut AES192Ctx, key24: *const u8);
    fn AES192_encrypt(ctx: *const AES192Ctx, blocks: usize, cipher: *mut u8, plain: *const u8);
    fn AES192_decrypt(ctx: *const AES192Ctx, blocks: usize, plain: *mut u8, cipher: *const u8);

    fn AES256_init(ctx: *mut AES256Ctx, key32: *const u8);
    fn AES256_encrypt(ctx: *const AES256Ctx, blocks: usize, cipher: *mut u8, plain: *const u8);
    fn AES256_decrypt(ctx: *const AES256Ctx, blocks: usize, plain: *mut u8, cipher: *const u8);

    fn AES128_CBC_init(ctx: *mut AES128CbcCtx, key16: *const u8, iv: *const u8);
    fn AES128_CBC_encrypt(ctx: *mut AES128CbcCtx, blocks: usize, out: *mut u8, inp: *const u8);
    fn AES128_CBC_decrypt(ctx: *mut AES128CbcCtx, blocks: usize, out: *mut u8, inp: *const u8);

    fn AES192_CBC_init(ctx: *mut AES192CbcCtx, key24: *const u8, iv: *const u8); // header typo: key16
    fn AES192_CBC_encrypt(ctx: *mut AES192CbcCtx, blocks: usize, out: *mut u8, inp: *const u8);
    fn AES192_CBC_decrypt(ctx: *mut AES192CbcCtx, blocks: usize, out: *mut u8, inp: *const u8);

    fn AES256_CBC_init(ctx: *mut AES256CbcCtx, key32: *const u8, iv: *const u8);
    fn AES256_CBC_encrypt(ctx: *mut AES256CbcCtx, blocks: usize, out: *mut u8, inp: *const u8);
    fn AES256_CBC_decrypt(ctx: *mut AES256CbcCtx, blocks: usize, out: *mut u8, inp: *const u8);
}

/// Macro spares the copy-paste.
macro_rules! impl_aes {
    ($name:ident, $Ctx:ident, $key_len:expr,
     $init:ident, $enc:ident, $dec:ident) => {
        pub struct $name {
            inner: $Ctx,
        }

        impl $name {
            /// Create a new context.
            pub fn new(key: &[u8; $key_len]) -> Self {
                let mut ctx = MaybeUninit::<$Ctx>::uninit();
                unsafe {
                    $init(ctx.as_mut_ptr(), key.as_ptr());
                    Self {
                        inner: ctx.assume_init(),
                    }
                }
            }

            /// Encrypts the input. The input must not be empty and its size must be a multiple of
            /// 16.  The output size must be the same as the input size.
            pub fn encrypt_to_slice(&self, plain: &[u8], cipher: &mut [u8]) {
                assert!(!plain.is_empty());
                assert_eq!(plain.len() % 16, 0, "input is not block-aligned");
                assert_eq!(
                    plain.len(),
                    cipher.len(),
                    "output must have the same size as the input"
                );
                unsafe {
                    $enc(
                        &self.inner,
                        plain.len() / 16,
                        cipher.as_mut_ptr(),
                        plain.as_ptr(),
                    )
                }
            }

            /// Encrypts the input. The input must not be empty and its size must be a multiple of
            /// 16.
            pub fn encrypt(&self, plain: &[u8]) -> Vec<u8> {
                assert!(!plain.is_empty());
                assert_eq!(plain.len() % 16, 0, "input is not block-aligned");
                let mut out = vec![0u8; plain.len()];
                self.encrypt_to_slice(plain, out.as_mut_slice());
                out
            }

            /// Decrypts the input. The input must not be empty and its size must be a multiple of
            /// 16.  The output size must be the same as the input size.
            pub fn decrypt_to_slice(&self, cipher: &[u8], plain: &mut [u8]) {
                assert!(!cipher.is_empty());
                assert_eq!(cipher.len() % 16, 0, "input is not block-aligned");
                assert_eq!(
                    plain.len(),
                    cipher.len(),
                    "output must have the same size as the input"
                );
                unsafe {
                    $dec(
                        &self.inner,
                        cipher.len() / 16,
                        plain.as_mut_ptr(),
                        cipher.as_ptr(),
                    )
                }
            }

            /// Decrypts the input. The input must not be empty and its size must be a multiple of
            /// 16.
            pub fn decrypt(&self, cipher: &[u8]) -> Vec<u8> {
                assert!(!cipher.is_empty());
                assert_eq!(cipher.len() % 16, 0, "input is not block-aligned");
                let mut out = vec![0u8; cipher.len()];
                self.decrypt_to_slice(cipher, out.as_mut_slice());
                out
            }
        }
    };
}

macro_rules! impl_aes_cbc {
    ($name:ident, $Ctx:ident, $key_len:expr,
     $init:ident, $enc:ident, $dec:ident) => {
        pub struct $name {
            inner: $Ctx,
        }

        impl $name {
            /// Create a new CBC context.
            pub fn new(key: &[u8; $key_len], iv: &[u8; 16]) -> Self {
                let mut ctx = MaybeUninit::<$Ctx>::uninit();
                unsafe {
                    $init(ctx.as_mut_ptr(), key.as_ptr(), iv.as_ptr());
                    Self {
                        inner: ctx.assume_init(),
                    }
                }
            }

            /// Encrypts the input. The input must not be empty and its size must be a multiple of
            /// 16.  The output size must be the same as the input size.
            pub fn encrypt_to_slice(&mut self, plain: &[u8], cipher: &mut [u8]) {
                assert_eq!(plain.len() % 16, 0, "input is not block-aligned");
                assert_eq!(
                    plain.len(),
                    cipher.len(),
                    "output must have the same size as the input"
                );
                unsafe {
                    $enc(
                        &mut self.inner,
                        plain.len() / 16,
                        cipher.as_mut_ptr(),
                        plain.as_ptr(),
                    )
                }
            }

            /// Encrypts the input. The input must not be empty and its size must be a multiple of
            /// 16.
            pub fn encrypt(&mut self, plain: &[u8]) -> Vec<u8> {
                assert_eq!(plain.len() % 16, 0, "input is not block-aligned");
                let mut out = vec![0u8; plain.len()];
                self.encrypt_to_slice(plain, out.as_mut_slice());
                out
            }

            /// Decrypts the input. The input must not be empty and its size must be a multiple of
            /// 16.  The output size must be the same as the input size.
            pub fn decrypt_to_slice(&mut self, cipher: &[u8], plain: &mut [u8]) {
                assert_eq!(cipher.len() % 16, 0, "input is not block-aligned");
                assert_eq!(
                    plain.len(),
                    cipher.len(),
                    "output must have the same size as the input"
                );
                unsafe {
                    $dec(
                        &mut self.inner,
                        cipher.len() / 16,
                        plain.as_mut_ptr(),
                        cipher.as_ptr(),
                    )
                }
            }

            /// Decrypts the input. The input must not be empty and its size must be a multiple of
            /// 16.
            pub fn decrypt(&mut self, cipher: &[u8]) -> Vec<u8> {
                assert_eq!(cipher.len() % 16, 0, "input is not block-aligned");
                let mut out = vec![0u8; cipher.len()];
                self.decrypt_to_slice(cipher, out.as_mut_slice());
                out
            }
        }
    };
}

impl_aes!(
    Aes128,
    AES128Ctx,
    16,
    AES128_init,
    AES128_encrypt,
    AES128_decrypt
);
impl_aes!(
    Aes192,
    AES192Ctx,
    24,
    AES192_init,
    AES192_encrypt,
    AES192_decrypt
);
impl_aes!(
    Aes256,
    AES256Ctx,
    32,
    AES256_init,
    AES256_encrypt,
    AES256_decrypt
);

impl_aes_cbc!(
    Aes128Cbc,
    AES128CbcCtx,
    16,
    AES128_CBC_init,
    AES128_CBC_encrypt,
    AES128_CBC_decrypt
);
impl_aes_cbc!(
    Aes192Cbc,
    AES192CbcCtx,
    24,
    AES192_CBC_init,
    AES192_CBC_encrypt,
    AES192_CBC_decrypt
);
impl_aes_cbc!(
    Aes256Cbc,
    AES256CbcCtx,
    32,
    AES256_CBC_init,
    AES256_CBC_encrypt,
    AES256_CBC_decrypt
);

#[cfg(test)]
mod tests {
    use super::*;
    use hex::decode;

    fn unhex(s: &str) -> Vec<u8> {
        decode(s).unwrap()
    }

    #[test]
    fn test_roundtrip() {
        let key = [0u8; 32];
        let plain = [0u8; 16];
        let aes = Aes256::new(&key);
        let cipher = aes.encrypt(&plain);
        assert_eq!(hex::encode(&cipher), "dc95c078a2408989ad48a21492842087");
        let dec = aes.decrypt(&cipher);
        assert_eq!(plain, dec.as_slice());
    }

    /* ---------- ECB vectors, taken from ctaes/test.c ---------- */

    struct Ecb<'a> {
        ks: usize,
        key: &'a str,
        plain: &'a str,
        cipher: &'a str,
    }

    const ECB: &[Ecb] = &[
        // FIPS-197 test vectors
        Ecb {
            ks: 128,
            key: "000102030405060708090a0b0c0d0e0f",
            plain: "00112233445566778899aabbccddeeff",
            cipher: "69c4e0d86a7b0430d8cdb78070b4c55a",
        },
        Ecb {
            ks: 192,
            key: "000102030405060708090a0b0c0d0e0f1011121314151617",
            plain: "00112233445566778899aabbccddeeff",
            cipher: "dda97ca4864cdfe06eaf70a0ec0d7191",
        },
        Ecb {
            ks: 256,
            key: "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
            plain: "00112233445566778899aabbccddeeff",
            cipher: "8ea2b7ca516745bfeafc49904b496089",
        },
        // NIST SP800-38A ECB
        Ecb {
            ks: 128,
            key: "2b7e151628aed2a6abf7158809cf4f3c",
            plain: "6bc1bee22e409f96e93d7e117393172a",
            cipher: "3ad77bb40d7a3660a89ecaf32466ef97",
        },
        Ecb {
            ks: 128,
            key: "2b7e151628aed2a6abf7158809cf4f3c",
            plain: "ae2d8a571e03ac9c9eb76fac45af8e51",
            cipher: "f5d3d58503b9699de785895a96fdbaaf",
        },
        Ecb {
            ks: 128,
            key: "2b7e151628aed2a6abf7158809cf4f3c",
            plain: "30c81c46a35ce411e5fbc1191a0a52ef",
            cipher: "43b1cd7f598ece23881b00e3ed030688",
        },
        Ecb {
            ks: 128,
            key: "2b7e151628aed2a6abf7158809cf4f3c",
            plain: "f69f2445df4f9b17ad2b417be66c3710",
            cipher: "7b0c785e27e8ad3f8223207104725dd4",
        },
        Ecb {
            ks: 192,
            key: "8e73b0f7da0e6452c810f32b809079e562f8ead2522c6b7b",
            plain: "6bc1bee22e409f96e93d7e117393172a",
            cipher: "bd334f1d6e45f25ff712a214571fa5cc",
        },
        Ecb {
            ks: 192,
            key: "8e73b0f7da0e6452c810f32b809079e562f8ead2522c6b7b",
            plain: "ae2d8a571e03ac9c9eb76fac45af8e51",
            cipher: "974104846d0ad3ad7734ecb3ecee4eef",
        },
        Ecb {
            ks: 192,
            key: "8e73b0f7da0e6452c810f32b809079e562f8ead2522c6b7b",
            plain: "30c81c46a35ce411e5fbc1191a0a52ef",
            cipher: "ef7afd2270e2e60adce0ba2face6444e",
        },
        Ecb {
            ks: 192,
            key: "8e73b0f7da0e6452c810f32b809079e562f8ead2522c6b7b",
            plain: "f69f2445df4f9b17ad2b417be66c3710",
            cipher: "9a4b41ba738d6c72fb16691603c18e0e",
        },
        Ecb {
            ks: 256,
            key: "603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4",
            plain: "6bc1bee22e409f96e93d7e117393172a",
            cipher: "f3eed1bdb5d2a03c064b5a7e3db181f8",
        },
        Ecb {
            ks: 256,
            key: "603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4",
            plain: "ae2d8a571e03ac9c9eb76fac45af8e51",
            cipher: "591ccb10d410ed26dc5ba74a31362870",
        },
        Ecb {
            ks: 256,
            key: "603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4",
            plain: "30c81c46a35ce411e5fbc1191a0a52ef",
            cipher: "b6ed21b99ca6f4f9f153e7b1beafed1d",
        },
        Ecb {
            ks: 256,
            key: "603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4",
            plain: "f69f2445df4f9b17ad2b417be66c3710",
            cipher: "23304b7a39f9f3ff067d8d8f9e24ecc7",
        },
    ];

    /* ---------- CBC vectors, taken from ctaes/test.c ---------- */

    struct Cbc<'a> {
        ks: usize,
        key: &'a str,
        iv: &'a str,
        plain: &'a str,
        cipher: &'a str,
    }

    const CBC: &[Cbc] = &[
        Cbc {
            ks: 128,
            key: "2b7e151628aed2a6abf7158809cf4f3c",
            iv: "000102030405060708090a0b0c0d0e0f",
            plain: "6bc1bee22e409f96e93d7e117393172a\
                    ae2d8a571e03ac9c9eb76fac45af8e51\
                    30c81c46a35ce411e5fbc1191a0a52ef\
                    f69f2445df4f9b17ad2b417be66c3710",
            cipher: "7649abac8119b246cee98e9b12e9197d\
                     5086cb9b507219ee95db113a917678b2\
                     73bed6b8e3c1743b7116e69e22229516\
                     3ff1caa1681fac09120eca307586e1a7",
        },
        Cbc {
            ks: 192,
            key: "8e73b0f7da0e6452c810f32b809079e562f8ead2522c6b7b",
            iv: "000102030405060708090a0b0c0d0e0f",
            plain: "6bc1bee22e409f96e93d7e117393172a\
                    ae2d8a571e03ac9c9eb76fac45af8e51\
                    30c81c46a35ce411e5fbc1191a0a52ef\
                    f69f2445df4f9b17ad2b417be66c3710",
            cipher: "4f021db243bc633d7178183a9fa071e8\
                     b4d9ada9ad7dedf4e5e738763f69145a\
                     571b242012fb7ae07fa9baac3df102e0\
                     08b0e27988598881d920a9e64f5615cd",
        },
        Cbc {
            ks: 256,
            key: "603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4",
            iv: "000102030405060708090a0b0c0d0e0f",
            plain: "6bc1bee22e409f96e93d7e117393172a\
                    ae2d8a571e03ac9c9eb76fac45af8e51\
                    30c81c46a35ce411e5fbc1191a0a52ef\
                    f69f2445df4f9b17ad2b417be66c3710",
            cipher: "f58c4c04d6e5f1ba779eabfb5f7bfbd6\
                     9cfc4e967edb808d679f777bc6702c7d\
                     39f23369a9d9bacfa530e26304231461\
                     b2eb05e2c39be9fcda6c19078c6a9d1b",
        },
    ];

    #[test]
    fn test_ecb() {
        for v in ECB {
            let key = unhex(v.key);
            let plain = unhex(v.plain);

            /* encrypt */
            let cipher = match v.ks {
                128 => {
                    let aes = Aes128::new(key.as_slice().try_into().unwrap());
                    aes.encrypt(&plain)
                }
                192 => {
                    let aes = Aes192::new(key.as_slice().try_into().unwrap());
                    aes.encrypt(&plain)
                }
                256 => {
                    let aes = Aes256::new(key.as_slice().try_into().unwrap());
                    aes.encrypt(&plain)
                }
                _ => unreachable!(),
            };
            assert_eq!(unhex(v.cipher), cipher);

            /* decrypt */
            let dec = match v.ks {
                128 => {
                    let aes = Aes128::new(key.as_slice().try_into().unwrap());
                    aes.decrypt(&cipher)
                }
                192 => {
                    let aes = Aes192::new(key.as_slice().try_into().unwrap());
                    aes.decrypt(&cipher)
                }
                256 => {
                    let aes = Aes256::new(key.as_slice().try_into().unwrap());
                    aes.decrypt(&cipher)
                }
                _ => unreachable!(),
            };
            assert_eq!(plain, dec);
        }
    }

    #[test]
    fn test_cbc() {
        for v in CBC {
            let key = unhex(v.key);
            let iv: [u8; 16] = unhex(v.iv).as_slice().try_into().unwrap();
            let plain = unhex(v.plain);

            /* encrypt */
            let cipher = match v.ks {
                128 => {
                    let mut cbc = Aes128Cbc::new(key.as_slice().try_into().unwrap(), &iv);
                    cbc.encrypt(&plain)
                }
                192 => {
                    let mut cbc = Aes192Cbc::new(key.as_slice().try_into().unwrap(), &iv);
                    cbc.encrypt(&plain)
                }
                256 => {
                    let mut cbc = Aes256Cbc::new(key.as_slice().try_into().unwrap(), &iv);
                    cbc.encrypt(&plain)
                }
                _ => unreachable!(),
            };
            assert_eq!(unhex(v.cipher), cipher);

            /* decrypt */
            let dec = match v.ks {
                128 => {
                    let mut cbc = Aes128Cbc::new(key.as_slice().try_into().unwrap(), &iv);
                    cbc.decrypt(&cipher)
                }
                192 => {
                    let mut cbc = Aes192Cbc::new(key.as_slice().try_into().unwrap(), &iv);
                    cbc.decrypt(&cipher)
                }
                256 => {
                    let mut cbc = Aes256Cbc::new(key.as_slice().try_into().unwrap(), &iv);
                    cbc.decrypt(&cipher)
                }
                _ => unreachable!(),
            };
            assert_eq!(plain, dec);
        }
    }
}
