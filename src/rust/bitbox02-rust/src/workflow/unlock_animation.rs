// Copyright 2025 Shift Crypto AG
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

use crate::bb02_async::screensaver_without;

/// Performs the unlock animation. Its duration is determined by the component render rate, see
/// unlock_animation.c
pub async fn animate() {
    let (sender, receiver) = async_channel::bounded(1);
    let mut component = bitbox02::ui::unlock_animation_create(move || {
        sender.try_send(()).unwrap();
    });
    component.screen_stack_push();
    screensaver_without(async move {
        match receiver.recv().await {
            Ok(()) => (),
            Err(_) => panic!("sender dropped"),
        }
    })
    .await;
}
