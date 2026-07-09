// SPDX-License-Identifier: Apache-2.0

use crate::hal::ui::ConfirmParams;
use alloc::vec::Vec;

use util::ascii;

pub enum Error {
    InvalidInput,
    UserAbort,
}

impl core::convert::From<crate::hal::ui::UserAbort> for Error {
    fn from(_error: crate::hal::ui::UserAbort) -> Self {
        Error::UserAbort
    }
}

/// Verify a message.
///
/// If the bytes are all printable ascii chars, the message is
/// confirmed one line at a time (the str is split into lines).
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
    if ascii::is_printable_ascii(msg, ascii::Charset::AllNewline) {
        // The message is all ascii and printable.
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
}
