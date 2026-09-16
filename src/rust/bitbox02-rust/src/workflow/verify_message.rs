// SPDX-License-Identifier: Apache-2.0

use crate::hal::ui::{ConfirmParams, Font};
use alloc::vec::Vec;

use crate::hal::Ui;

pub enum Error {
    InvalidInput,
    UserAbort,
}

impl core::convert::From<crate::hal::ui::UserAbort> for Error {
    fn from(_error: crate::hal::ui::UserAbort) -> Self {
        Error::UserAbort
    }
}

fn is_displayable_with_default_font(ui: &impl Ui, bytes: &[u8]) -> bool {
    let Ok(msg) = core::str::from_utf8(bytes) else {
        return false;
    };
    util::display::is_safe_text(msg, true)
        && msg
            .chars()
            .all(|c| c == '\n' || ui.has_glyph(Font::Default, c))
}

/// Verify a message.
///
/// If the bytes are valid UTF-8 and all codepoints are safe and covered by the default display
/// font, the message is confirmed one line at a time (the str is split into lines).
///
/// Otherwise, it is displayed as hex.
///
/// title_long is shown if it is only one line/screen.  title_short is shown if there are multiple
/// line screens, suffixed with the progress label (e.g. 1/3).
///
/// is_final if this is the final step in a workflow. In this case,
pub async fn verify(
    hal: &mut impl crate::hal::Hal,
    title_long: &str,
    title_short: &str,
    msg: &[u8],
    is_final: bool,
) -> Result<(), Error> {
    let is_displayable = {
        let ui = hal.ui();
        is_displayable_with_default_font(&*ui, msg)
    };

    if is_displayable {
        let msg = core::str::from_utf8(msg).unwrap();
        if msg.is_empty() {
            return Err(Error::InvalidInput);
        }

        let pages: Vec<&str> = msg.split('\n').collect();
        if pages.is_empty() {
            return Err(Error::InvalidInput);
        }
        for (i, &page) in pages.iter().enumerate() {
            let is_last = i == pages.len() - 1;
            let title = if pages.len() == 1 {
                title_long.into()
            } else {
                format!("{} {}/{}", title_short, i + 1, pages.len())
            };
            let params = ConfirmParams {
                title: &title,
                body: page,
                scrollable: true,
                accept_is_nextarrow: true, // longtouch takes priority over this if enabled
                longtouch: is_last && is_final,
                ..Default::default()
            };
            crate::workflow::confirm::confirm_value(hal, &params).await?;
        }
        Ok(())
    } else {
        let body = hex::encode(msg);
        let params = ConfirmParams {
            title: &format!("{}\ndata (hex)", title_long),
            body: &body,
            scrollable: true,
            display_size: msg.len(),
            accept_is_nextarrow: true, // longtouch takes priority over this if enabled
            longtouch: is_final,
            ..Default::default()
        };
        crate::workflow::confirm::confirm_value(hal, &params).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::boxed::Box;

    use crate::hal::testing::TestingHal;
    use crate::hal::testing::ui::Screen;
    use crate::workflow::confirm::{MAX_CONFIRM_BODY_SIZE, TRUNCATION_WARNING_BODY};

    #[async_test::test]
    async fn test_verify_multiline_text() {
        let mut hal = TestingHal::new();
        assert!(
            verify(&mut hal, "Sign message", "Sign", b"A\nB", true)
                .await
                .is_ok()
        );

        assert_eq!(
            hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Sign 1/2".into(),
                    body: "A".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Sign 2/2".into(),
                    body: "B".into(),
                    longtouch: true,
                },
            ]
        );
    }

    #[async_test::test]
    async fn test_verify_blank_lines() {
        let mut hal = TestingHal::new();
        assert!(
            verify(&mut hal, "Sign message", "Sign", b"A\n\nB", true)
                .await
                .is_ok()
        );
        assert_eq!(
            hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Sign 1/3".into(),
                    body: "A".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Sign 2/3".into(),
                    body: "".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Sign 3/3".into(),
                    body: "B".into(),
                    longtouch: true,
                },
            ]
        );

        let mut hal = TestingHal::new();
        assert!(
            verify(&mut hal, "Sign message", "Sign", b"\nA", true)
                .await
                .is_ok()
        );
        assert_eq!(
            hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Sign 1/2".into(),
                    body: "".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Sign 2/2".into(),
                    body: "A".into(),
                    longtouch: true,
                },
            ]
        );

        let mut hal = TestingHal::new();
        assert!(
            verify(&mut hal, "Sign message", "Sign", b"A\n", true)
                .await
                .is_ok()
        );
        assert_eq!(
            hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Sign 1/2".into(),
                    body: "A".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Sign 2/2".into(),
                    body: "".into(),
                    longtouch: true,
                },
            ]
        );
    }

    #[async_test::test]
    async fn test_verify_long_ascii_boundary() {
        let msg = "a".repeat(MAX_CONFIRM_BODY_SIZE);
        let mut hal = TestingHal::new();
        assert!(
            verify(&mut hal, "Sign message", "Sign", msg.as_bytes(), true)
                .await
                .is_ok()
        );
        assert_eq!(
            hal.ui.screens,
            vec![Screen::Confirm {
                title: "Sign message".into(),
                body: msg,
                longtouch: true,
            }]
        );

        let msg = "a".repeat(MAX_CONFIRM_BODY_SIZE + 1);
        let mut hal = TestingHal::new();
        assert!(
            verify(&mut hal, "Sign message", "Sign", msg.as_bytes(), true)
                .await
                .is_ok()
        );
        assert_eq!(
            hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Warning".into(),
                    body: TRUNCATION_WARNING_BODY.into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Sign message".into(),
                    body: msg,
                    longtouch: true,
                },
            ]
        );
    }

    #[async_test::test]
    async fn test_verify_multiline_warns_only_for_overlong_lines() {
        let overlong_line = "b".repeat(MAX_CONFIRM_BODY_SIZE + 1);
        let msg = format!("ok\n{overlong_line}");
        let mut hal = TestingHal::new();
        assert!(
            verify(&mut hal, "Sign message", "Sign", msg.as_bytes(), true)
                .await
                .is_ok()
        );
        assert_eq!(
            hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Sign 1/2".into(),
                    body: "ok".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Warning".into(),
                    body: TRUNCATION_WARNING_BODY.into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Sign 2/2".into(),
                    body: overlong_line,
                    longtouch: true,
                },
            ]
        );
    }

    #[async_test::test]
    async fn test_verify_binary_hex_boundary() {
        let mut hal = TestingHal::new();
        assert!(
            verify(&mut hal, "OP_RETURN", "OP_RETURN", &[0xff; 320], false)
                .await
                .is_ok()
        );
        assert_eq!(hal.ui.screens.len(), 1);
        assert_eq!(hal.ui.confirm_display_sizes, vec![320]);

        let mut hal = TestingHal::new();
        assert!(
            verify(&mut hal, "OP_RETURN", "OP_RETURN", &[0xff; 321], false)
                .await
                .is_ok()
        );
        assert_eq!(
            hal.ui.screens[0],
            Screen::Confirm {
                title: "Warning".into(),
                body: TRUNCATION_WARNING_BODY.into(),
                longtouch: false,
            }
        );
        assert_eq!(hal.ui.confirm_display_sizes, vec![0, 321]);
        match &hal.ui.screens[1] {
            Screen::Confirm { title, body, .. } => {
                assert_eq!(title, "OP_RETURN\ndata (hex)");
                assert_eq!(body.len(), MAX_CONFIRM_BODY_SIZE + 2);
            }
            _ => panic!("unexpected screen"),
        }
    }

    #[test]
    fn test_is_displayable_with_default_font() {
        let mut mock_hal = TestingHal::new();
        mock_hal.ui.set_has_glyph(Box::new(|_, _| true));
        assert!(is_displayable_with_default_font(
            &mock_hal.ui,
            "Zürich".as_bytes()
        ));
        assert!(is_displayable_with_default_font(
            &mock_hal.ui,
            "µ\nA".as_bytes()
        ));
        assert!(!is_displayable_with_default_font(
            &mock_hal.ui,
            "Aȑ".as_bytes()
        ));
        assert!(!is_displayable_with_default_font(
            &mock_hal.ui,
            "東京".as_bytes()
        ));
        assert!(!is_displayable_with_default_font(
            &mock_hal.ui,
            "tab\t".as_bytes()
        ));
        assert!(!is_displayable_with_default_font(
            &mock_hal.ui,
            "non\u{a0}breaking space".as_bytes()
        ));
        assert!(!is_displayable_with_default_font(
            &mock_hal.ui,
            "soft\u{ad}hyphen".as_bytes()
        ));
        assert!(!is_displayable_with_default_font(&mock_hal.ui, &[0xff]));

        mock_hal.ui.set_has_glyph(Box::new(|_, c| c != 'ü'));
        assert!(!is_displayable_with_default_font(
            &mock_hal.ui,
            "Zürich".as_bytes()
        ));
    }

    #[async_test::test]
    async fn test_verify_displayable_with_default_font() {
        let mut mock_hal = TestingHal::new();
        let result = verify(
            &mut mock_hal,
            "Sign message",
            "Sign",
            "Zürich\nµ".as_bytes(),
            true,
        )
        .await;
        assert!(matches!(result, Ok(())));

        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Sign 1/2".into(),
                    body: "Zürich".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Sign 2/2".into(),
                    body: "µ".into(),
                    longtouch: true,
                },
            ]
        );
        assert_eq!(mock_hal.ui.confirm_display_sizes, vec![0, 0]);
    }

    #[async_test::test]
    async fn test_verify_hex_if_not_displayable_with_default_font() {
        let mut mock_hal = TestingHal::new();
        let result = verify(
            &mut mock_hal,
            "Sign message",
            "Sign",
            "東京".as_bytes(),
            true,
        )
        .await;
        assert!(matches!(result, Ok(())));

        assert_eq!(
            mock_hal.ui.screens,
            vec![Screen::Confirm {
                title: "Sign message\ndata (hex)".into(),
                body: "e69db1e4baac".into(),
                longtouch: true,
            }]
        );
        assert_eq!(mock_hal.ui.confirm_display_sizes, vec![6]);
    }

    #[async_test::test]
    async fn test_verify_hex_if_glyph_missing_from_default_font() {
        let mut mock_hal = TestingHal::new();
        mock_hal.ui.set_has_glyph(Box::new(|_font, c| c != 'ȑ'));

        let result = verify(&mut mock_hal, "Sign message", "Sign", "Aȑ".as_bytes(), true).await;
        assert!(matches!(result, Ok(())));

        assert_eq!(
            mock_hal.ui.screens,
            vec![Screen::Confirm {
                title: "Sign message\ndata (hex)".into(),
                body: "41c891".into(),
                longtouch: true,
            }]
        );
        assert_eq!(mock_hal.ui.confirm_display_sizes, vec![3]);
    }
}
