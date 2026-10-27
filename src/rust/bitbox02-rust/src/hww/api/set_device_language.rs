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
    if language == current_language {
        return Ok(Response::Success(pb::Success {}));
    }
    if hal.memory().is_initialized() {
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
    }
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
        mock_hal.memory.set_initialized().unwrap();
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

    #[async_test::test]
    async fn test_process_before_initialized() {
        for seeded in [false, true] {
            let mut hal = TestingHal::new();
            if seeded {
                hal.memory
                    .set_encrypted_seed_and_hmac(
                        &[0; 96],
                        crate::hal::memory::PasswordStretchAlgo::V0,
                    )
                    .unwrap();
            }
            assert_eq!(hal.memory.is_seeded(), seeded);
            assert!(!hal.memory.is_initialized());

            for (code, language) in [("de", Language::German), ("en-US", Language::English)] {
                assert_eq!(
                    process(
                        &mut hal,
                        &pb::SetDeviceLanguageRequest {
                            language: code.into()
                        }
                    )
                    .await,
                    Ok(Response::Success(pb::Success {}))
                );
                assert_eq!(hal.memory.get_device_language(), language);
                assert!(hal.ui.screens.is_empty());
            }

            assert_eq!(
                process(
                    &mut hal,
                    &pb::SetDeviceLanguageRequest {
                        language: "fr".into()
                    }
                )
                .await,
                Err(Error::InvalidInput)
            );
            assert_eq!(hal.memory.get_device_language(), Language::English);
            assert!(hal.ui.screens.is_empty());
        }
    }

    #[async_test::test]
    async fn test_process_same_language() {
        for initialized in [false, true] {
            let mut hal = TestingHal::new();
            if initialized {
                hal.memory.set_initialized().unwrap();
            }
            for (code, language) in [("en-US", Language::English), ("de-DE", Language::German)] {
                hal.memory.set_device_language(language).unwrap();
                assert_eq!(
                    process(
                        &mut hal,
                        &pb::SetDeviceLanguageRequest {
                            language: code.into()
                        }
                    )
                    .await,
                    Ok(Response::Success(pb::Success {}))
                );
                assert_eq!(hal.memory.get_device_language(), language);
                assert!(hal.ui.screens.is_empty());
            }
        }
    }
}
