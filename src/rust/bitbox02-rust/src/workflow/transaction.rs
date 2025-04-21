// Copyright 2021 Shift Crypto AG
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

use crate::hal::Ui;

use crate::bb02_async::option_no_screensaver;
use core::cell::RefCell;

use alloc::boxed::Box;
use alloc::string::String;

pub struct UserAbort;

pub async fn verify_recipient(recipient: &str, amount: &str) -> Result<(), UserAbort> {
    let result = RefCell::new(None as Option<Result<(), UserAbort>>);

    let mut component = bitbox02::ui::confirm_transaction_address_create(
        amount,
        recipient,
        Box::new(|ok| {
            *result.borrow_mut() = Some(if ok { Ok(()) } else { Err(UserAbort) });
        }),
    );
    component.screen_stack_push();
    option_no_screensaver(&result).await
}

fn format_percentage(p: f64) -> String {
    let int: u64 = num_traits::float::FloatCore::round(p * 10.) as _;
    util::decimal::format_no_trim(int, 1)
}

pub async fn verify_total_fee(total: &str, fee: &str, longtouch: bool) -> Result<(), UserAbort> {
    let result = RefCell::new(None as Option<Result<(), UserAbort>>);

    let mut component = bitbox02::ui::confirm_transaction_fee_create(
        total,
        fee,
        longtouch,
        Box::new(|ok| {
            *result.borrow_mut() = Some(if ok { Ok(()) } else { Err(UserAbort) });
        }),
    );
    component.screen_stack_push();
    option_no_screensaver(&result).await
}

pub async fn verify_total_fee_maybe_warn(
    hal: &mut impl crate::hal::Hal,
    total: &str,
    fee: &str,
    fee_percentage: Option<f64>,
) -> Result<(), UserAbort> {
    const FEE_WARNING_THRESHOLD: f64 = 10.;
    let fee_percentage = fee_percentage.filter(|&f| f >= FEE_WARNING_THRESHOLD);
    let longtouch = fee_percentage.is_none();
    hal.ui().verify_total_fee(total, fee, longtouch).await?;

    if let Some(fee_percentage) = fee_percentage {
        match hal
            .ui()
            .confirm(&super::confirm::Params {
                title: "High fee",
                body: &format!(
                    "The fee is {}%\nthe send amount.\nProceed?",
                    format_percentage(fee_percentage)
                ),
                longtouch: true,
                ..Default::default()
            })
            .await
        {
            Ok(()) => (),
            Err(super::confirm::UserAbort) => return Err(UserAbort),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_percentage() {
        assert_eq!(format_percentage(0.), "0.0");
        assert_eq!(format_percentage(10.0), "10.0");
        assert_eq!(format_percentage(10.1), "10.1");
        assert_eq!(format_percentage(10.14), "10.1");
        assert_eq!(format_percentage(10.15), "10.2");
    }
}
