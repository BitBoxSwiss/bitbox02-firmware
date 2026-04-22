// SPDX-License-Identifier: Apache-2.0

use crate::hal::Ui;
use crate::hal::ui::ConfirmParams;
use crate::hal::ui::UserAbort;

use alloc::string::String;

fn format_percentage_tenths(tenths: u128) -> String {
    format!("{}.{}", tenths / 10, tenths % 10)
}

pub fn warning_fee_percentage(fee: u64, amount: u64) -> Option<String> {
    if amount == 0 {
        return None;
    }
    let fee = fee as u128;
    let amount = amount as u128;
    if fee * 10 < amount {
        return None;
    }
    Some(format_percentage_tenths((fee * 1000 + amount / 2) / amount))
}

fn format_percentage_text(fee_percentage: &str) -> String {
    format!("The fee is {}%\nthe send amount.\nProceed?", fee_percentage)
}

pub async fn verify_total_fee_maybe_warn(
    hal: &mut impl crate::hal::Hal,
    total: &str,
    fee: &str,
    fee_percentage: Option<&str>,
) -> Result<(), UserAbort> {
    let longtouch = fee_percentage.is_none();
    hal.ui().verify_total_fee(total, fee, longtouch).await?;

    if let Some(fee_percentage) = fee_percentage {
        hal.ui()
            .confirm(&ConfirmParams {
                title: "High fee",
                body: &format_percentage_text(fee_percentage),
                longtouch: true,
                ..Default::default()
            })
            .await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_warning_fee_percentage() {
        assert_eq!(warning_fee_percentage(1, 0), None);
        assert_eq!(warning_fee_percentage(0, 100), None);
        assert_eq!(warning_fee_percentage(9, 100), None);
        assert_eq!(warning_fee_percentage(3, 4), Some("75.0".into()));
        assert_eq!(warning_fee_percentage(10, 100), Some("10.0".into()));
        assert_eq!(warning_fee_percentage(101, 1000), Some("10.1".into()));
        assert_eq!(warning_fee_percentage(1014, 10000), Some("10.1".into()));
        assert_eq!(warning_fee_percentage(1015, 10000), Some("10.2".into()));
        assert_eq!(warning_fee_percentage(995, 10000), None);
        assert_eq!(warning_fee_percentage(909, 1000), Some("90.9".into()));
    }
}
