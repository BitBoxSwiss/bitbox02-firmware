// SPDX-License-Identifier: Apache-2.0

use super::Error;
use super::pb;
use crate::hal::ui::ConfirmParams;
use crate::i18n::I18n as _;

use pb::response::Response;

use crate::hal::Memory;
use crate::hal::Ui;

use crate::keystore;

use alloc::string::ToString;
use alloc::vec::Vec;

/// Processes a BIP-85 API call.
pub async fn process(
    hal: &mut impl crate::hal::Hal,
    request: &pb::Bip85Request,
) -> Result<Response, Error> {
    match &request.app {
        None => Err(Error::InvalidInput),
        Some(pb::bip85_request::App::Bip39(())) => Ok(Response::Bip85(pb::Bip85Response {
            app: Some(pb::bip85_response::App::Bip39(process_bip39(hal).await?)),
        })),
        Some(pb::bip85_request::App::Ln(request)) => Ok(Response::Bip85(pb::Bip85Response {
            app: Some(pb::bip85_response::App::Ln(process_ln(hal, request).await?)),
        })),
    }
}

/// Derives and displays a BIP-39 seed according to BIP-85:
/// https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#bip39.
async fn process_bip39(hal: &mut impl crate::hal::Hal) -> Result<(), Error> {
    use crate::hal::ui::{CanCancel, TrinaryChoice};

    let title = crate::tr!(hal, "BIP-85");
    let body = crate::tr!(hal, "Derive BIP-39\nmnemonic?");
    hal.ui()
        .confirm(&ConfirmParams {
            title: &title,
            body: &body,
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?;

    let title = crate::tr!(hal, "BIP-85");
    let body = crate::tr!(
        hal,
        "This is an advanced feature. Proceed only if you know what you are doing."
    );
    hal.ui()
        .confirm(&ConfirmParams {
            title: &title,
            body: &body,
            scrollable: true,
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?;

    let how_many_words = crate::tr!(hal, "How many words?");
    let num_words: u32 = match hal
        .ui()
        .trinary_choice(&how_many_words, Some("12"), None, Some("24"))
        .await
    {
        TrinaryChoice::Left => 12,
        TrinaryChoice::Middle => unreachable!(),
        TrinaryChoice::Right => 24,
    };

    let status = crate::tr_format!(hal, "{} words", &[&num_words.to_string()]);
    hal.ui().status(&status, true).await;

    // Pick index. The first few are quick-access. "More" leads to a full number input keyboard.
    let more = crate::tr!(hal, "More");
    let select_index = crate::tr!(hal, "Select index");
    let words = ["0", "1", "2", "3", "4", more.as_ref()];
    let index: u32 = match hal.ui().menu(&words, Some(&select_index)).await? {
        i @ 0..=4 => i.into(),
        5 => {
            let title = crate::tr!(hal, "Enter index");
            let number_string = hal
                .ui()
                .enter_string(
                    &crate::hal::ui::EnterStringParams {
                        title: &title,
                        number_input: true,
                        longtouch: true,
                        ..Default::default()
                    },
                    CanCancel::Yes,
                    "",
                )
                .await?;
            match number_string.as_str().parse::<u32>() {
                Ok(i) if i < util::bip32::HARDENED => i,
                _ => {
                    let status = crate::tr!(hal, "Invalid index");
                    hal.ui().status(&status, false).await;
                    return Err(Error::InvalidInput);
                }
            }
        }
        6.. => panic!("bip85 error"),
    };

    let status = crate::tr_format!(hal, "Index: {}", &[&index.to_string()]);
    hal.ui().status(&status, true).await;

    let title = crate::tr!(hal, "Keypath");
    hal.ui()
        .confirm(&ConfirmParams {
            title: &title,
            body: &format!("m/83696968'/39'/0'/{}'/{}'", num_words, index),
            scrollable: true,
            longtouch: true,
            ..Default::default()
        })
        .await?;

    let mnemonic = keystore::bip85_bip39(hal, num_words, index).await?;
    let words: Vec<&str> = mnemonic.split(' ').collect();
    {
        let language = hal.memory().get_device_language();
        let crate::hal::HalSubsystems { ui, random, .. } = hal.as_mut();
        crate::workflow::mnemonic::show_and_confirm_mnemonic(ui, random, &words, language).await?;
    }

    let status = crate::tr!(hal, "Finished");
    hal.ui().status(&status, true).await;

    Ok(())
}

/// Derives and displays a LN seed according to BIP-85.
/// It is the same as BIP-85 with app number 39', but instead using app number 19534' (= 0x4c4e = 'LN'),
/// and restricted to 12 word mnemonics.
/// https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#bip39
async fn process_ln(
    hal: &mut impl crate::hal::Hal,
    &pb::bip85_request::AppLn { account_number }: &pb::bip85_request::AppLn,
) -> Result<Vec<u8>, Error> {
    // We allow only one LN account until we see a reason to have more.
    if account_number != 0 {
        return Err(Error::InvalidInput);
    }
    let body = crate::tr!(hal, "Create\nLightning wallet\non host device?");
    hal.ui()
        .confirm(&ConfirmParams {
            title: "",
            body: &body,
            longtouch: true,
            ..Default::default()
        })
        .await?;

    Ok(keystore::bip85_ln(hal, account_number)
        .await
        .map_err(|_| Error::Generic)?
        .to_vec())
}
