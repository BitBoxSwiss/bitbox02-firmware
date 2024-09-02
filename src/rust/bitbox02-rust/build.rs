// Copyright 2024 Shift Crypto AG
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

// Emit a warning if FIRMWARE_VERSION_SHORT isn't set. We don't want this to be a hard error during
// development so that rust tools are happy.
fn main() {
    let version = option_env!("FIRMWARE_VERSION_SHORT");
    if let Some(version) = version {
        if version.is_empty() {
            println!("cargo::warning=FIRMWARE_VERSION_SHORT is empty");
        }
    } else {
        println!("cargo::warning=FIRMWARE_VERSION_SHORT is not set");
    }
}
