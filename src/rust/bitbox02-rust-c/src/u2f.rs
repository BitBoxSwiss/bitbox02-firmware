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

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

struct App {
    app_id: &'static [u8],
    name: &'static str,
}

const APPS: &[App] = &[
    App{
        // sha256('https://github.com/u2f/trusted_facets')
        app_id: b"\x70\x61\x7d\xfe\xd0\x65\x86\x3a\xf4\x7c\x15\x55\x6c\x91\x79\x88\x80\x82\x8c\xc4\x07\xfd\xf7\x0a\xe8\x50\x11\x56\x94\x65\xa0\x75",
        name: "GitHub",
    },
    App{
        // ('https://www.gstatic.com/securitykey/origins.json')
        app_id: b"\xa5\x46\x72\xb2\x22\xc4\xcf\x95\xe1\x51\xed\x8d\x4d\x3c\x76\x7a\x6c\xc3\x49\x43\x59\x43\x79\x4e\x88\x4f\x3d\x02\x3a\x82\x29\xfd",
        name: "Google",
    },
    App{
        // sha256('https://bitbucket.org')
        app_id: b"\x12\x74\x3b\x92\x12\x97\xb7\x7f\x11\x35\xe4\x1f\xde\xdd\x4a\x84\x6a\xfe\x82\xe1\xf3\x69\x32\xa9\x91\x2f\x3b\x0d\x8d\xfb\x7d\x0e",
        name: "Bitbucket",
    },
    App{
        // sha256('https://www.bitfinex.com')
        app_id: b"\x30\x2f\xd5\xb4\x49\x2a\x07\xb9\xfe\xbb\x30\xe7\x32\x69\xec\xa5\x01\x20\x5c\xcf\xe0\xc2\x0b\xf7\xb4\x72\xfa\x2d\x31\xe2\x1e\x63",
        name: "Bitfinex",
    },
    App{
        // sha256('https://vault.bitwarden.com/app-id.json')
        app_id: b"\xa3\x4d\x30\x9f\xfa\x28\xc1\x24\x14\xb8\xba\x6c\x07\xee\x1e\xfa\xe1\xa8\x5e\x8a\x04\x61\x48\x59\xa6\x7c\x04\x93\xb6\x95\x61\x90",
        name: "Bitwarden",
    },
    App{
        // sha256('https://www.dashlane.com')
        app_id: b"\x68\x20\x19\x15\xd7\x4c\xb4\x2a\xf5\xb3\xcc\x5c\x95\xb9\x55\x3e\x3e\x3a\x83\xb4\xd2\xa9\x3b\x45\xfb\xad\xaa\x84\x69\xff\x8e\x6e",
        name: "Dashlane",
    },
    App{
        // sha256('https://www.dropbox.com/u2f-app-id.json')
        app_id: b"\xc5\x0f\x8a\x7b\x70\x8e\x92\xf8\x2e\x7a\x50\xe2\xbd\xc5\x5d\x8f\xd9\x1a\x22\xfe\x6b\x29\xc0\xcd\xf7\x80\x55\x30\x84\x2a\xf5\x81",
        name: "Dropbox",
    },
    App{
        // sha256('https://www.fastmail.com')
        app_id: b"\x69\x66\xab\xe3\x67\x4e\xa2\xf5\x30\x79\xeb\x71\x01\x97\x84\x8c\x9b\xe6\xf3\x63\x99\x2f\xd0\x29\xe9\x89\x84\x47\xcb\x9f\x00\x84",
        name: "FastMail",
    },
    App{
        // sha256('https://id.fedoraproject.org/u2f-origins.json')
        app_id: b"\x9d\x61\x44\x2f\x5c\xe1\x33\xbd\x46\x54\x4f\xc4\x2f\x0a\x6d\x54\xc0\xde\xb8\x88\x40\xca\xc2\xb6\xae\xfa\x65\x14\xf8\x93\x49\xe9",
        name: "Fedora",
    },
    App{
        // sha256('https://account.gandi.net/api/u2f/trusted_facets.json')
        app_id: b"\xa4\xe2\x2d\xca\xfe\xa7\xe9\x0e\x12\x89\x50\x11\x39\x89\xfc\x45\x97\x8d\xc9\xfb\x87\x76\x75\x60\x51\x6c\x1c\x69\xdf\xdf\xd1\x96",
        name: "Gandi",
    },
    App{
        // sha256('https://gitlab.com')
        app_id: b"\xe7\xbe\x96\xa5\x1b\xd0\x19\x2a\x72\x84\x0d\x2e\x59\x09\xf7\x2b\xa8\x2a\x2f\xe9\x3f\xaa\x62\x4f\x03\x39\x6b\x30\xe4\x94\xc8\x04",
        name: "GitLab",
    },
    App{
        // sha256('https://keepersecurity.com')
        app_id: b"\x53\xa1\x5b\xa4\x2a\x7c\x03\x25\xb8\xdb\xee\x28\x96\x34\xa4\x8f\x58\xae\xa3\x24\x66\x45\xd5\xff\x41\x8f\x9b\xb8\x81\x98\x85\xa9",
        name: "Keeper",
    },
    App{
        // sha256('https://slushpool.com/static/security/u2f.json')
        app_id: b"\x08\xb2\xa3\xd4\x19\x39\xaa\x31\x66\x84\x93\xcb\x36\xcd\xcc\x4f\x16\xc4\xd9\xb4\xc8\x23\x8b\x73\xc2\xf6\x72\xc0\x33\x00\x71\x97",
        name: "Slush Pool",
    },
    App{
        // sha256('https://dashboard.stripe.com')
        app_id: b"\x2a\xc6\xad\x09\xa6\xd0\x77\x2c\x44\xda\x73\xa6\x07\x2f\x9d\x24\x0f\xc6\x85\x4a\x70\xd7\x9c\x10\x24\xff\x7c\x75\x59\x59\x32\x92",
        name: "Stripe",
    },
    App{
        // sha256('https://u2f.bin.coffee')
        app_id: b"\x1b\x3c\x16\xdd\x2f\x7c\x46\xe2\xb4\xc2\x89\xdc\x16\x74\x6b\xcc\x60\xdf\xcf\x0f\xb8\x18\xe1\x32\x15\x52\x6e\x14\x08\xe7\xf4\x68",
        name: "u2f.bin.coffee",
    },
    App{
        // sha256('https://u2f.aws.amazon.com/app-id.json')
        app_id: b"\x96\x89\x78\xa2\x99\x53\xde\x52\xd3\xef\x0f\x0c\x71\xb7\xb7\xb6\xb1\xaf\x9f\x08\xe2\x57\x89\x6a\x8d\x81\x26\x91\x85\x30\x29\x3b",
        name: "Amazon Web Services",
    },
    App{
        // sha256('https://tutanota.com/u2f-appid.json')
        app_id: b"\xfa\xbe\xec\xe3\x98\x2f\xad\x9d\xdc\xc9\x8f\x91\xbd\x2e\x75\xaf\xc7\xd1\xf4\xca\x54\x49\x29\xb2\xd0\xd0\x42\x12\xdf\xfa\x30\xfa",
        name: "Tutanota",
    },
];

/// Returns the site name/identifier to be displayed during U2F registration/auth.
fn app_string(app_id: &[u8; 32]) -> String {
    if let Some(&App { name, .. }) = APPS.iter().find(|app| app.app_id == app_id) {
        return name.into();
    }

    let m = bip39::Mnemonic::from_entropy_in(bip39::Language::English, app_id).unwrap();
    let words: Vec<&'static str> = m.words().collect();
    alloc::format!(
        "Unknown site:\n{} {}\n{} {}",
        words[0],
        words[1],
        words[2],
        words[3],
    )
}

// app_id must be of length 32, and out must be a least 60 bytes.
#[unsafe(no_mangle)]
pub extern "C" fn rust_u2f_app_string(app_id: crate::util::Bytes, mut out: crate::util::BytesMut) {
    let app_str = app_string(app_id.as_ref().try_into().unwrap());
    let bytes = app_str.as_bytes();
    let out = out.as_mut();
    out[..bytes.len()].clone_from_slice(bytes);
    out[bytes.len()] = 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_u2f_app_string_known() {
        let app_id = b"\xc5\x0f\x8a\x7b\x70\x8e\x92\xf8\x2e\x7a\x50\xe2\xbd\xc5\x5d\x8f\xd9\x1a\x22\xfe\x6b\x29\xc0\xcd\xf7\x80\x55\x30\x84\x2a\xf5\x81";
        let mut string = [0u8; 100];
        rust_u2f_app_string(
            unsafe { crate::util::rust_util_bytes(app_id.as_ptr(), app_id.len()) },
            unsafe { crate::util::rust_util_bytes_mut(string.as_mut_ptr(), string.len()) },
        );
        assert_eq!(
            core::ffi::CStr::from_bytes_until_nul(&string)
                .unwrap()
                .to_str()
                .unwrap(),
            "Dropbox"
        );
    }

    #[test]
    fn test_rust_u2f_app_string_unknown() {
        let app_id = b"\x24\x1d\x5b\x78\x35\x90\xc2\x1f\x79\x69\x8e\x7c\xe8\x92\xdd\x03\xfb\x2c\x8f\xad\xc2\x44\x0e\xc2\x3a\xa5\xde\x9e\x2d\x23\x81\xb0";
        let mut string = [0u8; 100];
        rust_u2f_app_string(
            unsafe { crate::util::rust_util_bytes(app_id.as_ptr(), app_id.len()) },
            unsafe { crate::util::rust_util_bytes_mut(string.as_mut_ptr(), string.len()) },
        );
        assert_eq!(
            core::ffi::CStr::from_bytes_until_nul(&string)
                .unwrap()
                .to_str()
                .unwrap(),
            "Unknown site:\ncatch turn\ntask hen"
        );
    }
}
