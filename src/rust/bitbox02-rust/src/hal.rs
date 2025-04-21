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

use crate::workflow::RealWorkflows;
pub use crate::workflow::Workflows as Ui;

pub trait Sd {
    fn sdcard_inserted(&mut self) -> bool;
}

/// Hardware abstraction layer for BitBox devices.
pub trait Hal {
    fn ui(&mut self) -> &mut impl Ui;
    fn sd(&mut self) -> &mut impl Sd;
}

pub struct BitBox02Sd;

impl Sd for BitBox02Sd {
    fn sdcard_inserted(&mut self) -> bool {
        bitbox02::sd::sdcard_inserted()
    }
}

pub struct BitBox02Hal {
    ui: RealWorkflows,
    sd: BitBox02Sd,
}

impl BitBox02Hal {
    pub const fn new() -> Self {
        Self {
            ui: crate::workflow::RealWorkflows,
            sd: BitBox02Sd,
        }
    }
}

impl Hal for BitBox02Hal {
    fn ui(&mut self) -> &mut impl Ui {
        &mut self.ui
    }
    fn sd(&mut self) -> &mut impl Sd {
        &mut self.sd
    }
}

#[cfg(feature = "testing")]
pub mod testing {
    pub struct TestingSd {
        pub inserted: Option<bool>,
    }

    impl TestingSd {
        pub fn new() -> Self {
            Self { inserted: None }
        }
    }
    pub struct TestingHal<'a> {
        pub ui: crate::workflow::testing::TestingWorkflows<'a>,
        pub sd: TestingSd,
    }

    impl super::Sd for TestingSd {
        fn sdcard_inserted(&mut self) -> bool {
            self.inserted.unwrap()
        }
    }

    impl TestingHal<'_> {
        pub fn new() -> Self {
            Self {
                ui: crate::workflow::testing::TestingWorkflows::new(),
                sd: TestingSd::new(),
            }
        }
    }

    impl super::Hal for TestingHal<'_> {
        fn ui(&mut self) -> &mut impl super::Ui {
            &mut self.ui
        }
        fn sd(&mut self) -> &mut impl super::Sd {
            &mut self.sd
        }
    }
}
