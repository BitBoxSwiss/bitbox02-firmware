// SPDX-License-Identifier: Apache-2.0

use crate::hal::Ui;

use alloc::string::String;

pub struct UserAbort;

pub async fn verify_recipient(recipient: &str, amount: &str) -> Result<(), UserAbort> {
    if bitbox02::ui::confirm_transaction_address_create(amount, recipient).await {
        Ok(())
    } else {
        Err(UserAbort)
    }
}

fn format_percentage(p: f64) -> String {
    let int: u64 = num_traits::float::FloatCore::round(p * 10.) as _;
    util::decimal::format_no_trim(int, 1)
}

pub async fn verify_total_fee(total: &str, fee: &str, longtouch: bool) -> Result<(), UserAbort> {
    if bitbox02::ui::confirm_transaction_fee_create(total, fee, longtouch).await {
        Ok(())
    } else {
        Err(UserAbort)
    }
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
