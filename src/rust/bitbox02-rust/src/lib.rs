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
#![no_std]

use core::fmt::Write;
use core::panic::PanicInfo;
use core::time::Duration;

mod general;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print_debug!(0, "Internal error: {}", info);
    loop {}
}
