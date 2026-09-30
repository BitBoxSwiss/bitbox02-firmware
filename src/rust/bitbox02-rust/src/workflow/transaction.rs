// SPDX-License-Identifier: Apache-2.0

use crate::hal::Ui;
use crate::hal::ui::ConfirmParams;
use crate::hal::ui::UserAbort;

use alloc::string::String;

fn format_percentage(p: f64) -> String {
    let int: u64 = num_traits::float::FloatCore::round(p * 10.) as _;
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
    fn warning_message(&self, fee_percentage: &str) -> String {
        match self {
            FeePercentageBasis::SendAmount => {
                format!("The fee is {}%\nthe send amount.\nProceed?", fee_percentage)
            }
            FeePercentageBasis::TotalInputs => {
                format!("The fee is {}%\nof all inputs.\nProceed?", fee_percentage)
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
        let warning_message =
            fee_percentage_basis.warning_message(&format_percentage(fee_percentage));
        hal.ui()
            .confirm(&ConfirmParams {
                title: "High fee",
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
    }
}
