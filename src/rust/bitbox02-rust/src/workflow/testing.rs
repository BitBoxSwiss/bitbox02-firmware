// Copyright 2025 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::{confirm, transaction, Workflows};

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Eq, PartialEq)]
pub enum Screen {
    Confirm {
        title: String,
        body: String,
        longtouch: bool,
    },
    TotalFee {
        total: String,
        fee: String,
        longtouch: bool,
    },
    Recipient {
        recipient: String,
        amount: String,
    },
    Status {
        title: String,
        success: bool,
    },
    More,
}

/// An Workflows implementation for unit tests. Collects all screens and provides helper functions
/// to verify them.
pub struct TestingWorkflows {
    _abort_nth: Option<usize>,
    pub screens: Vec<Screen>,
}

impl Workflows for TestingWorkflows {
    async fn confirm(&mut self, params: &confirm::Params<'_>) -> Result<(), confirm::UserAbort> {
        self.screens.push(Screen::Confirm {
            title: params.title.into(),
            body: params.body.into(),
            longtouch: params.longtouch,
        });
        if self
            ._abort_nth
            .as_ref()
            .is_some_and(|&n| self.screens.len() - 1 == n)
        {
            return Err(confirm::UserAbort);
        }
        Ok(())
    }

    async fn verify_recipient(
        &mut self,
        recipient: &str,
        amount: &str,
    ) -> Result<(), transaction::UserAbort> {
        self.screens.push(Screen::Recipient {
            recipient: recipient.into(),
            amount: amount.into(),
        });
        if self
            ._abort_nth
            .as_ref()
            .is_some_and(|&n| self.screens.len() - 1 == n)
        {
            return Err(transaction::UserAbort);
        }
        Ok(())
    }

    async fn verify_total_fee(
        &mut self,
        total: &str,
        fee: &str,
        longtouch: bool,
    ) -> Result<(), transaction::UserAbort> {
        self.screens.push(Screen::TotalFee {
            total: total.into(),
            fee: fee.into(),
            longtouch,
        });
        if self
            ._abort_nth
            .as_ref()
            .is_some_and(|&n| self.screens.len() - 1 == n)
        {
            return Err(transaction::UserAbort);
        }
        Ok(())
    }

    async fn status(&mut self, title: &str, status_success: bool) {
        self.screens.push(Screen::Status {
            title: title.into(),
            success: status_success,
        });
        if self
            ._abort_nth
            .as_ref()
            .is_some_and(|&n| self.screens.len() - 1 == n)
        {
            panic!("canot abort status screen");
        }
    }
}

impl TestingWorkflows {
    pub fn new() -> Self {
        Self {
            screens: vec![],
            _abort_nth: None,
        }
    }

    /// Make the `n`-th workflow (0-indexed) fail with a user abort. If that workflow cannot be
    /// aborted, there will be panic.
    pub fn abort_nth(&mut self, n: usize) {
        self._abort_nth = Some(n);
    }

    pub fn contains_confirm(&self, confirm_title: &str, confirm_body: &str) -> bool {
        self.screens.iter().any(|screen| match screen {
            Screen::Confirm { title, body, .. } => title == confirm_title && body == confirm_body,
            _ => false,
        })
    }
}
