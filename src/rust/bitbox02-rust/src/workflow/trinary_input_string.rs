// SPDX-License-Identifier: Apache-2.0

pub use crate::hal::ui::UserAbort as Error;
pub use bitbox02::ui::{TrinaryInputStringParams as Params, trinary_input_string};

#[derive(Copy, Clone)]
pub enum CanCancel {
    No,
    Yes,
}
