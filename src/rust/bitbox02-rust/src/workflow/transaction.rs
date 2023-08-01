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

use crate::bb02_async::option;
use core::cell::RefCell;

use alloc::boxed::Box;

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
    option(&result).await
}

pub async fn verify_total_fee(
    total: &str,
    fee: &str,
    fee_percentage: Option<f64>,
) -> Result<(), UserAbort> {
    let result = RefCell::new(None as Option<Result<(), UserAbort>>);

    const FEE_WARNING_THRESHOLD: f64 = 10.;
    let fee_percentage = fee_percentage.filter(|&f| f >= FEE_WARNING_THRESHOLD);
    let longtouch = fee_percentage.is_none();
    let mut component = bitbox02::ui::confirm_transaction_fee_create(
        total,
        fee,
        longtouch,
        Box::new(|ok| {
            *result.borrow_mut() = Some(if ok { Ok(()) } else { Err(UserAbort) });
        }),
    );
    component.screen_stack_push();
    option(&result).await?;

    if let Some(fee_percentage) = fee_percentage {
        match super::confirm::confirm(&super::confirm::Params {
            title: "High fee",
            body: &format!("The fee is {:.1}%\nthe send amount.\nProceed?", fee_percentage),
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
