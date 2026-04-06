// SPDX-License-Identifier: Apache-2.0

use super::Error;
use super::pb;
use crate::hal::Ui;
use crate::hal::ui::ConfirmParams;

use alloc::string::String;

use pb::solana_response::Response;

pub fn from_pubkey(pubkey: &[u8; 32]) -> String {
    bitcoin::base58::encode(pubkey)
}

fn format_display_address(address: &str) -> String {
    util::strings::format_address(address)
}

pub async fn process(
    hal: &mut impl crate::hal::Hal,
    request: &pb::SolanaPubRequest,
) -> Result<Response, Error> {
    super::keypath::validate(&request.keypath)?;
    let pubkey = super::derive_pubkey(hal, &request.keypath)?;
    let address = from_pubkey(&pubkey);

    if request.display {
        let displayed_address = format_display_address(&address);
        hal.ui()
            .confirm(&ConfirmParams {
                title: "Solana",
                body: &displayed_address,
                scrollable: true,
                ..Default::default()
            })
            .await?;
    }

    Ok(Response::Pub(pb::SolanaPubResponse {
        address,
        pubkey: pubkey.to_vec(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hal::testing::TestingHal;
    use crate::hal::testing::ui::Screen;
    use crate::keystore::testing::mock_unlocked;
    use util::bb02_async::block_on;
    use util::bip32::HARDENED;

    #[test]
    fn test_format_display_address() {
        assert_eq!(
            format_display_address("5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp"),
            "5eyk t4Us Fv8P 8NJd TREp Y1vz qKqZ Kvdp"
        );
    }

    #[test]
    fn test_process() {
        mock_unlocked();
        let request = pb::SolanaPubRequest {
            keypath: vec![44 + HARDENED, 501 + HARDENED, HARDENED, HARDENED],
            display: false,
        };

        let response = block_on(process(&mut TestingHal::new(), &request)).unwrap();
        let Response::Pub(pb::SolanaPubResponse { address, pubkey }) = response else {
            panic!("unexpected response");
        };
        assert_eq!(pubkey.len(), 32);
        assert_eq!(address, from_pubkey(pubkey.as_slice().try_into().unwrap()));

        let mut hal = TestingHal::new();
        let response = block_on(process(
            &mut hal,
            &pb::SolanaPubRequest {
                display: true,
                ..request
            },
        ))
        .unwrap();
        let Response::Pub(pb::SolanaPubResponse { address, .. }) = response else {
            panic!("unexpected response");
        };
        assert_eq!(
            hal.ui.screens,
            vec![Screen::Confirm {
                title: "Solana".into(),
                body: format_display_address(&address),
                longtouch: false,
            }]
        );
    }
}
