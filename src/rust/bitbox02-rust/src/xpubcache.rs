// Copyright 2023 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::keystore;

use crate::bip32;
use alloc::vec::Vec;

pub trait Xpub: Sized {
    /// Derives a child xpub using the provided keypath.
    fn derive(&self, keypath: &[u32]) -> Result<Self, ()>;

    /// Derives an xpub from the root xpub using the provided keypath.
    fn from_keypath(keypath: &[u32]) -> Result<Self, ()>;
}

/// Implements a cache for xpubs. Cached intermediate xpubs are used to derive child xpubs.
///
/// The cache must be configured using `cache_keypath()`, otherwise no caching occurs. The reason
/// for this is that automatic caching is harder to get right and reason about, e.g. in a BTC tx, we
/// shouldn't cache xpubs at the address level (e.g. m/84/0'/0'/0/0), as they don't repeat and there
/// can be many of them.
pub struct XpubCache<X> {
    // List of keypaths for which we want to cache xpubs for.
    keypaths: Vec<Vec<u32>>,
    // Cached xpubs. First tuple element is the keypath for which the xpub was cached, the second
    // element is the cached xpub.
    xpubs: Vec<(Vec<u32>, X)>,
}

impl<X: Xpub + Clone> XpubCache<X> {
    pub fn new() -> Self {
        XpubCache {
            keypaths: Vec::new(),
            xpubs: Vec::new(),
        }
    }

    /// Instruct the cache that we want to cache the xpub at this keypath. The xpub is not derived
    /// yet, it will be derived when requested for the first time using `get_xpub()`.
    pub fn add_keypath(&mut self, keypath: &[u32]) {
        self.keypaths.push(keypath.to_vec());
    }

    // Retrieves a cached xpub. If the xpub is not cached, derive and cache it first.
    fn cache_get_set(&mut self, keypath: &[u32]) -> Result<X, ()> {
        // Return cached xpub if exists.
        if let Some((_, xpub)) = self
            .xpubs
            .iter()
            .find(|&(cached_keypath, _)| cached_keypath == keypath)
        {
            return Ok(xpub.clone());
        }

        // Otherwise, compute xpub and cache it.
        //
        // Before deriving the xpub from the root, try to derive it from an intermediate cached
        // xpub. Only if the last element is not hardened, as we can only derive unhardened elements
        // from an xpub (hardened elements require the xprv).
        const UNHARDENED_LAST: u32 = util::bip32::HARDENED - 1;
        let xpub = if let [prefix @ .., last @ 0..=UNHARDENED_LAST] = keypath {
            self.get_xpub(prefix)?.derive(&[*last])?
        } else {
            X::from_keypath(keypath)?
        };
        self.xpubs.push((keypath.to_vec(), xpub.clone()));
        Ok(xpub)
    }

    /// Derive an xpub from the keystore's master key. If a prefix of the keypath is cached, the
    /// cached xpub will be used as basis for derivation. The longest cached prefix (shortest
    /// suffix) is used to minimize the number child derivations necessary afterwards.
    pub fn get_xpub(&mut self, keypath: &[u32]) -> Result<X, ()> {
        // Check if any prefix of keypath is is marked as cached. Get the longest such prefix.
        let search_result = self
            .keypaths
            .iter()
            .filter_map(|kp| {
                keypath
                    .strip_prefix(kp.as_slice())
                    .map(|suffix| (kp, suffix))
            })
            .max_by_key(|(kp, _)| kp.len());
        if let Some((cached_prefix, suffix)) = search_result {
            let xpub = self.cache_get_set(&cached_prefix.clone())?;
            return xpub.derive(suffix);
        }
        X::from_keypath(keypath)
    }
}

impl Xpub for bip32::Xpub {
    fn derive(&self, keypath: &[u32]) -> Result<Self, ()> {
        bip32::Xpub::derive(self, keypath)
    }

    fn from_keypath(keypath: &[u32]) -> Result<Self, ()> {
        keystore::get_xpub(keypath)
    }
}

pub type Bip32XpubCache = XpubCache<bip32::Xpub>;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::bip32;
    use bitbox02::testing::mock_unlocked;
    use util::bip32::HARDENED;

    #[test]
    fn test_xpub_cache() {
        // Mock xpubs by storing the keypath only, so we can unit test access patterns.
        #[derive(Clone)]
        struct MockXpub(Vec<u32>);

        static CHILD_DERIVATIONS: bitbox02::testing::UnsafeSyncRefCell<u32> =
            bitbox02::testing::UnsafeSyncRefCell::new(0);
        static ROOT_DERIVATIONS: bitbox02::testing::UnsafeSyncRefCell<u32> =
            bitbox02::testing::UnsafeSyncRefCell::new(0);

        impl Xpub for MockXpub {
            fn derive(&self, keypath: &[u32]) -> Result<Self, ()> {
                let mut kp = Vec::new();
                kp.extend_from_slice(&self.0);
                kp.extend_from_slice(keypath);
                *CHILD_DERIVATIONS.borrow_mut() += keypath.len() as u32;
                Ok(MockXpub(kp))
            }

            fn from_keypath(keypath: &[u32]) -> Result<Self, ()> {
                *ROOT_DERIVATIONS.borrow_mut() += 1;
                Ok(MockXpub(keypath.to_vec()))
            }
        }

        type MockCache = XpubCache<MockXpub>;

        let mut cache = MockCache::new();

        assert_eq!(cache.get_xpub(&[]).unwrap().0.as_slice(), &[]);
        assert_eq!(*CHILD_DERIVATIONS.borrow(), 0u32);
        assert_eq!(*ROOT_DERIVATIONS.borrow(), 1u32);
        *ROOT_DERIVATIONS.borrow_mut() = 0;

        assert_eq!(cache.get_xpub(&[1, 2, 3]).unwrap().0.as_slice(), &[1, 2, 3]);
        assert_eq!(*CHILD_DERIVATIONS.borrow(), 0u32);
        assert_eq!(*ROOT_DERIVATIONS.borrow(), 1u32);
        *ROOT_DERIVATIONS.borrow_mut() = 0;

        // Cache some keypaths.
        cache.add_keypath(&[84 + HARDENED, 0 + HARDENED, 0 + HARDENED]);
        cache.add_keypath(&[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 1]);

        assert_eq!(
            cache
                .get_xpub(&[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 1, 2])
                .unwrap()
                .0
                .as_slice(),
            &[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 1, 2]
        );
        // Two child derivations:
        // 1: m/84'/0'/0' -> m/84'/0'/0'/1
        // 2: m/84'/0'/0'/1 -> m/84'/0'/0'/1/2
        assert_eq!(*CHILD_DERIVATIONS.borrow(), 2u32);
        *CHILD_DERIVATIONS.borrow_mut() = 0;
        assert_eq!(*ROOT_DERIVATIONS.borrow(), 1u32);
        *ROOT_DERIVATIONS.borrow_mut() = 0;

        // Same keypath again is a cache hit at m/84'/0'/0'/1 with one child derivation.
        assert_eq!(
            cache
                .get_xpub(&[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 1, 2])
                .unwrap()
                .0
                .as_slice(),
            &[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 1, 2]
        );
        assert_eq!(*CHILD_DERIVATIONS.borrow(), 1u32);
        *CHILD_DERIVATIONS.borrow_mut() = 0;
        assert_eq!(*ROOT_DERIVATIONS.borrow(), 0u32);

        // m/84'/0'/0'/0/0 is a cache hit at m/84'/0'/0', which was cached because of the above we
        // call using m/84'/0'/0'/1/2.
        assert_eq!(
            cache
                .get_xpub(&[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0])
                .unwrap()
                .0
                .as_slice(),
            &[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0]
        );
        assert_eq!(*CHILD_DERIVATIONS.borrow(), 2u32);
        assert_eq!(*ROOT_DERIVATIONS.borrow(), 0u32);
    }

    #[test]
    fn test_bip32_xpub_cache() {
        let mut cache = Bip32XpubCache::new();
        cache.add_keypath(&[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 1]);
        cache.add_keypath(&[84 + HARDENED, 0 + HARDENED, 0 + HARDENED]);

        mock_unlocked();
        assert_eq!(
            &cache
                .get_xpub(&[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 1, 2])
                .unwrap()
                .serialize_str(bip32::XPubType::Xpub)
                .unwrap(),
            "xpub6H18r9myxw9MztwzVyBYj26X1gVkz9ZzwJ8UgV9HWYu4ae6NQ6AEs2ibibhbF6oK3bzduzVNv4gwmu78o4Z4tkdzAcDMp6siTFbVegg9DEi",
        );

        // Make sure the following xpubs are derived using the cache only, not touching the seed.
        bitbox02::keystore::lock();

        assert_eq!(
            &cache
                .get_xpub(&[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0])
                .unwrap()
                .serialize_str(bip32::XPubType::Xpub)
                .unwrap(),
            "xpub6GugPDcUhrSudznFss7wXvQV3gwFTEanxHdCyoNoHnZEr3PTbh2Fosg4JjfphaYAsqjBhmtTZ3Yo8tmGjSHtaPhExNiMCSvPzreqjrX4Wr7",
        );

        assert_eq!(
            &cache
                .get_xpub(&[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 1, 3])
                .unwrap()
                .serialize_str(bip32::XPubType::Xpub)
                .unwrap(),
            "xpub6H18r9myxw9N57ip5QtLS7m78pMKKpWM18Mzj9rCjgec4SPndPWT5C8EvjxtGBoWhpdNRizdeii73qKPZT2YgqxjVr5xyZuVT979N7pmGAS",
        );
    }
}
