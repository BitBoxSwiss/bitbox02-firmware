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

// Needed to link the simulator executable in /test/simulator, which link to
// bitbox_merged-simulator. `rust_eh_personality` is provided by Rust when building the firmware or
// running Rust unit tests.
void rust_eh_personality(void);
void rust_eh_personality(void) {}
