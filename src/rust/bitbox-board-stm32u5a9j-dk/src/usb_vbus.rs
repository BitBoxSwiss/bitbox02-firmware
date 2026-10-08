// SPDX-License-Identifier: Apache-2.0

// MB1829: PG1/ADC4_IN8 senses VBUS through 330 kOhm / 27 kOhm,
// with VREF+ = 1.8 V. A 5 V cable produces only about 0.38 V on PG1.
// See MB1829-U5A9NJQ-B01 schematic, sheet 9 (USB HS Type-C).
const fn adc_count(vbus_mv: u32) -> u16 {
    (vbus_mv * 4095 * 27 / (1800 * (330 + 27))) as u16
}

// Hysteresis avoids repeated attach/detach events while VBUS rises or falls.
const ATTACH: u16 = adc_count(4000);
const DETACH: u16 = adc_count(3600);

pub fn present(sample: u16, was_present: bool) -> bool {
    if was_present {
        sample >= DETACH
    } else {
        sample > ATTACH
    }
}

/// ADC watchdog interrupts when a sample leaves this inclusive window.
pub fn watchdog_window(present: bool) -> (u16, u16) {
    if present { (DETACH, 4095) } else { (0, ATTACH) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_present() {
        // About 860 counts at 5 V VBUS: far below a GPIO input's high threshold.
        assert!(present(860, false));
        assert!(present(817, false)); // 4.75 V
        assert!(!present(0, true));
        assert!(!present(172, false)); // 1 V residual voltage after unplug

        // A slow supply ramp or noise between the thresholds retains the state.
        assert!(!present(650, false));
        assert!(present(650, true));
        assert!(!present(ATTACH, false));
        assert!(present(ATTACH + 1, false));
        assert!(present(DETACH, true));
        assert!(!present(DETACH - 1, true));
    }

    #[test]
    fn test_watchdog_window() {
        // An interrupt must be generated precisely when the state can change.
        for was_present in [false, true] {
            let (low, high) = watchdog_window(was_present);
            for sample in 0..=4095 {
                assert_eq!(
                    present(sample, was_present) != was_present,
                    sample < low || sample > high,
                );
            }
        }
    }
}
