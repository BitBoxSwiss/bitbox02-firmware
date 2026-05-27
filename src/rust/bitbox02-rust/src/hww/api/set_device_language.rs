// SPDX-License-Identifier: Apache-2.0

use super::Error;
use crate::hal::ui::ConfirmParams;
use crate::hal::{Memory, Ui};
use crate::pb;

use bitbox_hal::memory::Language;
use pb::response::Response;

pub async fn process(
    hal: &mut impl crate::hal::Hal,
    pb::SetDeviceLanguageRequest { language }: &pb::SetDeviceLanguageRequest,
) -> Result<Response, Error> {
    let language = crate::i18n::language_from_code(language).ok_or(Error::InvalidInput)?;
    let current_language = hal.memory().get_device_language();
    let language_name = language_name(current_language, language);
    let body = crate::i18n::format(
        current_language,
        "Change language\nto {}?",
        &[language_name.as_ref()],
    );
    hal.ui()
        .confirm(&ConfirmParams {
            body: &body,
            ..Default::default()
        })
        .await?;
    hal.memory().set_device_language(language)?;
    Ok(Response::Success(pb::Success {}))
}

fn language_name(
    current_language: Language,
    language: Language,
) -> alloc::borrow::Cow<'static, str> {
    match language {
        Language::English => crate::i18n::translate(current_language, "English"),
        Language::German => crate::i18n::translate(current_language, "German"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hal::testing::TestingHal;
    use crate::hal::testing::ui::Screen;

    #[async_test::test]
    async fn test_set_device_language() {
        let mut mock_hal = TestingHal::new();
        assert_eq!(
            process(
                &mut mock_hal,
                &pb::SetDeviceLanguageRequest {
                    language: "de".into()
                }
            )
            .await,
            Ok(Response::Success(pb::Success {}))
        );
        assert_eq!(
            mock_hal.ui.screens,
            vec![Screen::Confirm {
                title: "".into(),
                body: "Change language\nto German?".into(),
                longtouch: false,
            }]
        );
        assert_eq!(
            mock_hal.memory.get_device_language(),
            bitbox_hal::memory::Language::German
        );

        assert_eq!(
            process(
                &mut mock_hal,
                &pb::SetDeviceLanguageRequest {
                    language: "en-US".into()
                }
            )
            .await,
            Ok(Response::Success(pb::Success {}))
        );
        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "".into(),
                    body: "Change language\nto German?".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "".into(),
                    body: "Sprache ändern\nauf Englisch?".into(),
                    longtouch: false,
                },
            ]
        );
        assert_eq!(
            mock_hal.memory.get_device_language(),
            bitbox_hal::memory::Language::English
        );

        mock_hal.ui.abort_nth(2);
        assert_eq!(
            process(
                &mut mock_hal,
                &pb::SetDeviceLanguageRequest {
                    language: "de".into()
                }
            )
            .await,
            Err(Error::UserAbort)
        );
        assert_eq!(
            mock_hal.memory.get_device_language(),
            bitbox_hal::memory::Language::English
        );

        assert_eq!(
            process(
                &mut mock_hal,
                &pb::SetDeviceLanguageRequest {
                    language: "fr".into()
                }
            )
            .await,
            Err(Error::InvalidInput)
        );
    }
}
