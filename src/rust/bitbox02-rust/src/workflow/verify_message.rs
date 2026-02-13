// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;

use super::confirm;
use crate::hal::Ui;

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

        let pages: Vec<&str> = msg.split('\n').filter(|line| !line.is_empty()).collect();
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
            let params = confirm::Params {
                title: &title,
                body: page,
                scrollable: true,
                accept_is_nextarrow: true, // longtouch takes priority over this if enabled
                longtouch: is_last && is_final,
                ..Default::default()
            };
            hal.ui().confirm(&params).await?;
        }
        Ok(())
    } else {
        let params = confirm::Params {
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
