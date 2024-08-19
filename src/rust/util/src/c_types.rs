// Copyright 2019 Shift Cryptosecurity AG
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

#![allow(non_camel_case_types)]
#[repr(u8)]
pub enum c_void {
    #[doc(hidden)]
    __variant,
}
pub type c_char = u8;
pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;

#[cfg(target_arch = "x86_64")]
pub type c_uint = u64;
#[cfg(target_arch = "aarch64")]
pub type c_uint = u64;
#[cfg(target_arch = "arm")]
pub type c_uint = u32;

#[cfg(target_arch = "x86_64")]
pub type c_long = i64;
#[cfg(target_arch = "aarch64")]
pub type c_long = i64;
#[cfg(target_arch = "arm")]
pub type c_long = i32;

#[cfg(target_arch = "x86_64")]
pub type c_ulong = u64;
#[cfg(target_arch = "aarch64")]
pub type c_ulong = u64;
#[cfg(target_arch = "arm")]
pub type c_ulong = u32;

pub type c_longlong = i64;
pub type c_ulonglong = u64;
