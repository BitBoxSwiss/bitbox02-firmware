// SPDX-License-Identifier: Apache-2.0

use crate::hal::Ui;
use crate::hal::ui::ConfirmParams;
use crate::hal::ui::UserAbort;
use crate::i18n::I18n as _;

use alloc::string::String;

fn format_percentage(p: f64) -> String {
    let scaled = p * 10.;
    let truncated = scaled as u64;
    let int = if scaled - truncated as f64 >= 0.5 {
        truncated.saturating_add(1)
    } else {
        truncated
    };
    util::decimal::format_no_trim(int, 1)
}

/// The denominator used to calculate and describe a transaction's fee percentage.
pub enum FeePercentageBasis {
    /// The amount sent to non-change/send outputs.
    SendAmount,
    /// The sum of the transaction's verified input values.
    TotalInputs,
}

impl FeePercentageBasis {
    fn warning_message(&self, hal: &mut impl crate::hal::Hal, fee_percentage: &str) -> String {
        match self {
            FeePercentageBasis::SendAmount => {
                crate::tr_format!(
                    hal,
                    "Fee is {}%\nof the send amount.\nProceed?",
                    &[fee_percentage]
                )
            }
            FeePercentageBasis::TotalInputs => {
                crate::tr_format!(
                    hal,
                    "Fee is {}%\nof all inputs.\nProceed?",
                    &[fee_percentage]
                )
            }
        }
    }
}

pub async fn verify_total_fee_maybe_warn(
    hal: &mut impl crate::hal::Hal,
    total: &str,
    fee: &str,
    fee_percentage: Option<f64>,
) -> Result<(), UserAbort> {
    verify_total_fee_maybe_warn_with_basis(
        hal,
        total,
        fee,
        fee_percentage,
        FeePercentageBasis::SendAmount,
    )
    .await
}

pub async fn verify_total_fee_maybe_warn_with_basis(
    hal: &mut impl crate::hal::Hal,
    total: &str,
    fee: &str,
    fee_percentage: Option<f64>,
    fee_percentage_basis: FeePercentageBasis,
) -> Result<(), UserAbort> {
    const FEE_WARNING_THRESHOLD: f64 = 10.;
    let fee_percentage = fee_percentage.filter(|&f| f >= FEE_WARNING_THRESHOLD);
    let longtouch = fee_percentage.is_none();
    hal.ui().verify_total_fee(total, fee, longtouch).await?;

    if let Some(fee_percentage) = fee_percentage {
        let title = crate::tr!(hal, "High fee");
        let warning_message =
            fee_percentage_basis.warning_message(hal, &format_percentage(fee_percentage));
        hal.ui()
            .confirm(&ConfirmParams {
                title: &title,
                body: &warning_message,
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
    fn test_format_percentage() {
        assert_eq!(format_percentage(0.), "0.0");
        assert_eq!(format_percentage(10.0), "10.0");
        assert_eq!(format_percentage(10.1), "10.1");
        assert_eq!(format_percentage(10.14), "10.1");
        assert_eq!(format_percentage(10.15), "10.2");
        assert_eq!(format_percentage(10.15_f64.next_down()), "10.1");
        assert_eq!(format_percentage(10.15_f64.next_up()), "10.2");
        assert_eq!(format_percentage(99.95), "100.0");
    }

    #[test]
    fn test_format_percentage_large_values() {
        assert_eq!(
            format_percentage(450_359_962_737_049.7),
            "450359962737049.7"
        );
        assert_eq!(
            format_percentage((u64::MAX as f64).next_down() / 10.),
            "1844674407370954956.8"
        );
        for p in [u64::MAX as f64 / 10., f64::MAX, f64::INFINITY] {
            assert_eq!(format_percentage(p), "1844674407370955161.5");
        }
    }

    #[test]
    fn test_format_percentage_negative_and_nan() {
        for p in [-0.05, -10.15, f64::NEG_INFINITY, f64::NAN] {
            assert_eq!(format_percentage(p), "0.0");
        }
    }
}
