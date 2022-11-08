// Copyright 2022 Shift Crypto AG
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

// Needed to link the C unit test executables in /test/unit-test, which link to bitbox_merged.
// `rust_eh_personality` is provided by Rust when building the firmware or running Rust unit tests
//
// See
// https://doc.rust-lang.org/unstable-book/language-features/lang-items.html#writing-an-executable-without-stdlib.
//
// One could get rid of this and also considerably shrink the binary size by compiling core instead
// of using pre-built binaries. See a proof of concept implementation here:
// https://github.com/digitalbitbox/bitbox02-firmware/tree/build-std-PoC.  We decided against doing
// this for now as the feature seems immature and because of the warnings against using it in
// production:
// https://github.com/rust-lang/wg-cargo-std-aware/tree/81765f0eb744b9c47840c16f43a32c9f61fd7f0c#mvp-implementation
void rust_eh_personality(void);
void rust_eh_personality(void) {}
