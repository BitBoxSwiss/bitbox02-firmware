// Copyright 2020 Shift Cryptosecurity AG
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

pub use bitbox02_sys::securechip_model_t as Model;

pub fn attestation_sign(challenge: &[u8; 32], signature: &mut [u8; 64]) -> Result<(), ()> {
    match unsafe {
        bitbox02_sys::securechip_attestation_sign(challenge.as_ptr(), signature.as_mut_ptr())
    } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn monotonic_increments_remaining() -> Result<u32, ()> {
    let mut result: u32 = 0;
    match unsafe { bitbox02_sys::securechip_monotonic_increments_remaining(&mut result as _) } {
        true => Ok(result),
        false => Err(()),
    }
}

#[cfg(feature = "app-u2f")]
#[cfg(not(feature = "testing"))]
pub fn u2f_counter_set(counter: u32) -> Result<(), ()> {
    match unsafe { bitbox02_sys::securechip_u2f_counter_set(counter) } {
        true => Ok(()),
        false => Err(()),
    }
}

#[cfg(feature = "app-u2f")]
#[cfg(feature = "testing")]
pub fn u2f_counter_set(_counter: u32) -> Result<(), ()> {
    Ok(())
}

pub fn model() -> Result<Model, ()> {
    let mut ver = core::mem::MaybeUninit::uninit();
    match unsafe { bitbox02_sys::securechip_model(ver.as_mut_ptr()) } {
        true => Ok(unsafe { ver.assume_init() }),
        false => Err(()),
    }
}
