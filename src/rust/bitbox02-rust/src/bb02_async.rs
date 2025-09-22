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

use core::cell::RefCell;

#[cfg(feature = "testing")]
pub use util::bb02_async::block_on;
pub use util::bb02_async::{Task, option, spin};

/// Disables the screensaver while waiting for an option to contain a value. Afterwards, it returns that value
pub async fn option_no_screensaver<O>(opt: &RefCell<Option<O>>) -> O {
    bitbox02::screen_saver::screen_saver_disable();
    let result = option(opt).await;
    bitbox02::screen_saver::screen_saver_enable();
    result
}
