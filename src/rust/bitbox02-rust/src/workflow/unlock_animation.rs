// SPDX-License-Identifier: Apache-2.0

/// Performs the unlock animation. Its duration is determined by the component render rate, see
/// unlock_animation.c
pub async fn animate() {
    bitbox02::ui::unlock_animation().await;
}
