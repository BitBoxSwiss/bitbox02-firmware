// SPDX-License-Identifier: Apache-2.0

use crate::hal::ui::{ConfirmParams, Font};
use alloc::{borrow::Cow, string::String, vec::Vec};

use crate::hal::Ui;

// Keep this in sync with src/ui/components/label.h:MAX_LABEL_SIZE.
const MAX_LABEL_SIZE: usize = 640;
const TRUNCATION_SUFFIX: &str = "...";

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
    msg.chars()
        .all(|c| c == '\n' || (!c.is_control() && ui.has_glyph(Font::Default, c)))
}

fn truncate_message_page(page: &str) -> Cow<'_, str> {
    if page.len() <= MAX_LABEL_SIZE {
        return Cow::Borrowed(page);
    }

    let mut end = MAX_LABEL_SIZE - TRUNCATION_SUFFIX.len();
    while !page.is_char_boundary(end) {
        end -= 1;
    }
    let mut result = String::with_capacity(MAX_LABEL_SIZE);
    result.push_str(&page[..end]);
    result.push_str(TRUNCATION_SUFFIX);
    Cow::Owned(result)
}

/// Verify a message.
///
/// If the bytes are valid UTF-8 and all codepoints are covered by the default display font, the
/// message is confirmed one line at a time (the str is split into lines).
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

        let pages: Vec<&str> = msg.split('\n').filter(|line| !line.is_empty()).collect();
        if pages.is_empty() {
            return Err(Error::InvalidInput);
        }
        for (i, &page) in pages.iter().enumerate() {
            let body = truncate_message_page(page);
            let is_last = i == pages.len() - 1;
            let title = if pages.len() == 1 {
                title_long.into()
            } else {
                format!("{} {}/{}", title_short, i + 1, pages.len())
            };
            let params = ConfirmParams {
                title: &title,
                body: body.as_ref(),
                scrollable: true,
                accept_is_nextarrow: true, // longtouch takes priority over this if enabled
                longtouch: is_last && is_final,
                ..Default::default()
            };
            hal.ui().confirm(&params).await?;
        }
        Ok(())
    } else {
        let params = ConfirmParams {
            title: &format!("{}\ndata (hex)", title_long),
            body: &hex::encode(msg),
            scrollable: true,
            display_size: msg.len(),
            accept_is_nextarrow: true, // longtouch takes priority over this if enabled
            longtouch: is_final,
            ..Default::default()
        };
        hal.ui().confirm(&params).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::boxed::Box;

    use crate::hal::testing::TestingHal;
    use crate::hal::testing::ui::Screen;

    #[test]
    fn test_is_displayable_with_default_font() {
        let mock_hal = TestingHal::new();
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
        assert!(!is_displayable_with_default_font(&mock_hal.ui, &[0xff]));
    }

    #[test]
    fn test_truncate_message_page() {
        assert_eq!(truncate_message_page("short").as_ref(), "short");

        let ascii = "a".repeat(MAX_LABEL_SIZE + 1);
        let ascii_truncated = truncate_message_page(&ascii);
        assert_eq!(ascii_truncated.len(), MAX_LABEL_SIZE);
        assert_eq!(
            ascii_truncated.as_ref(),
            format!(
                "{}{}",
                "a".repeat(MAX_LABEL_SIZE - TRUNCATION_SUFFIX.len()),
                TRUNCATION_SUFFIX
            )
        );

        let prefix = "a".repeat(MAX_LABEL_SIZE - TRUNCATION_SUFFIX.len() - 1);
        let utf8 = format!("{}übbbb", prefix);
        let utf8_truncated = truncate_message_page(&utf8);
        assert!(utf8_truncated.len() <= MAX_LABEL_SIZE);
        assert_eq!(
            utf8_truncated.as_ref(),
            format!("{}{}", prefix, TRUNCATION_SUFFIX)
        );
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
    async fn test_verify_truncates_displayable_utf8_on_char_boundary() {
        let prefix = "a".repeat(MAX_LABEL_SIZE - TRUNCATION_SUFFIX.len() - 1);
        let message = format!("{}übbbb", prefix);
        let expected_body = format!("{}{}", prefix, TRUNCATION_SUFFIX);

        let mut mock_hal = TestingHal::new();
        let result = verify(
            &mut mock_hal,
            "Sign message",
            "Sign",
            message.as_bytes(),
            true,
        )
        .await;
        assert!(matches!(result, Ok(())));

        assert!(expected_body.len() <= MAX_LABEL_SIZE);
        assert_eq!(
            mock_hal.ui.screens,
            vec![Screen::Confirm {
                title: "Sign message".into(),
                body: expected_body,
                longtouch: true,
            }]
        );
        assert_eq!(mock_hal.ui.confirm_display_sizes, vec![0]);
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
