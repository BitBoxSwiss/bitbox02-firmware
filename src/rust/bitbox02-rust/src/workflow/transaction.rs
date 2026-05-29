// SPDX-License-Identifier: Apache-2.0

use crate::hal::Ui;
use crate::hal::ui::ConfirmParams;
use crate::hal::ui::UserAbort;

use alloc::string::{String, ToString};

fn next_decimal_digit(remainder: u64, amount: u64) -> (u8, u64) {
    // Long-division step: compute floor(remainder * 10 / amount) without ever multiplying by 10,
    // as `remainder * 10` can overflow for arbitrary u64 inputs.
    let mut digit = 0;
    let mut next_remainder = 0;
    for _ in 0..10 {
        if next_remainder >= amount - remainder {
            next_remainder -= amount - remainder;
            digit += 1;
        } else {
            next_remainder += remainder;
        }
    }
    (digit, next_remainder)
}

fn rounded_fractional_tenths(mut remainder: u64, amount: u64) -> u16 {
    // Percent with one decimal is stored as tenths of a percent:
    // fee / amount * 100% * 10 = fee / amount * 1000.
    // The integer quotient is handled separately, so extract exactly the three fractional decimal
    // digits contributed by `remainder / amount`, then round from the leftover remainder.
    let mut result = 0;
    for _ in 0..3 {
        let (digit, next_remainder) = next_decimal_digit(remainder, amount);
        result = result * 10 + u16::from(digit);
        remainder = next_remainder;
    }
    if remainder >= amount - remainder {
        result += 1;
    }
    result
}

fn push_digit(out: &mut String, digit: u16) {
    out.push((b'0' + digit as u8) as char);
}

fn format_percentage(quotient: u64, fractional_tenths: u16) -> String {
    // `quotient * 100 + fractional_tenths / 10` can overflow u64 for extreme inputs. Format it as
    // decimal text instead: append two percent digits to `quotient`, then append the decimal digit.
    let mut whole = if fractional_tenths / 10 == 100 {
        // Rounding can turn 99.95% of the fractional part into an additional full `quotient`.
        let mut whole = (quotient + 1).to_string();
        whole.push('0');
        whole.push('0');
        whole
    } else {
        let tens = fractional_tenths / 100;
        let ones = (fractional_tenths / 10) % 10;
        let mut whole = if quotient == 0 {
            String::new()
        } else {
            quotient.to_string()
        };
        if quotient != 0 || tens != 0 {
            push_digit(&mut whole, tens);
        }
        push_digit(&mut whole, ones);
        whole
    };
    push_digit(&mut whole, fractional_tenths % 10);
    whole.insert(whole.len() - 1, '.');
    whole
}

pub fn warning_fee_percentage(fee: u64, amount: u64) -> Option<String> {
    if amount == 0 {
        return None;
    }
    let warning_threshold = amount / 10 + u64::from(amount % 10 != 0);
    if fee < warning_threshold {
        return None;
    }
    Some(format_percentage(
        fee / amount,
        rounded_fractional_tenths(fee % amount, amount),
    ))
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
        assert_eq!(
            warning_fee_percentage(u64::MAX, 1),
            Some("1844674407370955161500.0".into())
        );
        assert_eq!(
            warning_fee_percentage(u64::MAX, 2),
            Some("922337203685477580750.0".into())
        );
        assert_eq!(
            warning_fee_percentage(u64::MAX, u64::MAX),
            Some("100.0".into())
        );
        assert_eq!(
            warning_fee_percentage(u64::MAX - 1, u64::MAX),
            Some("100.0".into())
        );
    }
}
