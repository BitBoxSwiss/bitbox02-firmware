// SPDX-License-Identifier: Apache-2.0

use super::common::SOME_XPUB;
use crate::btc_transaction::{Coin, Outcome, Screen, VersionExpectation};

const TBTC_EXTERNAL_ADDRESS: &str =
    "tb1pff8vkq80pu2cgtu7ttgad2znw62v2lguhw6ptrppwns6nrpqau2qcuz37d";
const TBTC_EXTERNAL_ADDRESS_GROUPED: &str =
    "tb1p ff8v kq80 pu2c gtu7 ttga d2zn w62v 2lgu hw6p trpp wns6 nrpq au2q cuz3 7d";
const BTC_P2TR_ADDRESS: &str = "bc1pmg5dhafms6h9nts4dtehgkanym6yeccfmk5hx3ts3jxnm4zh2knqv80ha5";
const BTC_P2TR_ADDRESS_GROUPED: &str =
    "bc1p mg5d hafm s6h9 nts4 dteh gkan ym6y eccf mk5h x3ts 3jxn m4zh 2knq v80h a5";
const LTC_EXTERNAL_ADDRESS: &str = "ltc1qw508d6qejxtdg4y5r3zarvary0c5xw7kgmn4n9";
const LTC_EXTERNAL_ADDRESS_GROUPED: &str = "ltc1 qw50 8d6q ejxt dg4y 5r3z arva ry0c 5xw7 kgmn 4n9";
const SILENT_PAYMENT_ADDRESS: &str = "sp1qqgste7k9hx0qftg6qmwlkqtwuy6cycyavzmzj85c6qdfhjdpdjtdgqjuexzk6murw56suy3e0rd2cgqvycxttddwsvgxe2usfpxumr70xc9pkqwv";
const SILENT_PAYMENT_ADDRESS_GROUPED: &str = "sp1q qgst e7k9 hx0q ftg6 qmwl kqtw uy6c ycya vzmz j85c 6qdf hjdp djtd gqju exzk 6mur w56s uy3e 0rd2 cgqv ycxt tddw svgx e2us fpxu mr70 xc9p kqwv";
const PSBT_SAME_ACCOUNT_ADDRESS: &str = "tb1ql34ny8mcpgjqr0ngsnjmlpzjpgncyz2ygh2gye";
const PSBT_SAME_ACCOUNT_ADDRESS_GROUPED: &str =
    "tb1q l34n y8mc pgjq r0ng snjm lpzj pgnc yz2y gh2g ye";
const PSBT_OTHER_ACCOUNT_ADDRESS: &str = "tb1qvrcm2akp30d7ecnqdjk8qdu09962ak005rcp6j";
const PSBT_OTHER_ACCOUNT_ADDRESS_GROUPED: &str =
    "tb1q vrcm 2akp 30d7 ecnq djk8 qdu0 9962 ak00 5rcp 6j";
pub const DEVICE_POLICY_XPUB: &str = "[4c00739d/48'/1'/0'/3']tpubDF5MSzQdK2GfjmkNvrCZzpJhFt3if1HmrAdimugmGqWDCXYpkjxHpFZYuDxYYDAnnFMLMjLkMGvij2XV8pLtHBejgGy5RvNW4875nFGBDWv";

fn success(
    min_version: Option<&str>,
    max_version_exclusive: Option<&str>,
    screens: Vec<Screen>,
) -> VersionExpectation {
    VersionExpectation {
        min_version: min_version.map(Into::into),
        max_version_exclusive: max_version_exclusive.map(Into::into),
        outcome: Outcome::Success,
        unsupported_version: None,
        screens,
    }
}

fn unsupported_before(version: &str) -> VersionExpectation {
    VersionExpectation {
        min_version: None,
        max_version_exclusive: Some(version.into()),
        outcome: Outcome::Unsupported,
        unsupported_version: Some(version.into()),
        screens: vec![],
    }
}

fn invalid_input_before(version: &str) -> VersionExpectation {
    invalid_input(None, Some(version))
}

fn invalid_input(
    min_version: Option<&str>,
    max_version_exclusive: Option<&str>,
) -> VersionExpectation {
    invalid_input_with_screens(min_version, max_version_exclusive, vec![])
}

fn invalid_input_with_screens(
    min_version: Option<&str>,
    max_version_exclusive: Option<&str>,
    screens: Vec<Screen>,
) -> VersionExpectation {
    VersionExpectation {
        min_version: min_version.map(Into::into),
        max_version_exclusive: max_version_exclusive.map(Into::into),
        outcome: Outcome::InvalidInput,
        unsupported_version: None,
        screens,
    }
}

fn status() -> Screen {
    Screen::Status {
        title: "Transaction".into(),
        body: "confirmed".into(),
    }
}

fn address(amount: &str, address: &str) -> Screen {
    Screen::TransactionAddress {
        amount: amount.into(),
        address: address.into(),
    }
}

fn final_fee(amount: &str, fee: &str) -> Screen {
    Screen::TransactionFee {
        amount: amount.into(),
        fee: fee.into(),
        longtouch: true,
    }
}

fn warning_fee(amount: &str, fee: &str) -> Screen {
    Screen::TransactionFee {
        amount: amount.into(),
        fee: fee.into(),
        longtouch: false,
    }
}

fn high_fee(percent: u32) -> Screen {
    high_fee_decimal(&format!("{percent}.0"))
}

fn high_fee_decimal(percent: &str) -> Screen {
    Screen::Confirm {
        title: "High fee".into(),
        body: format!("The fee is {percent}%\nthe send amount.\nProceed?"),
        longtouch: true,
    }
}

fn confirmed_screens(high_fee_percent: &str) -> Vec<Screen> {
    vec![high_fee_decimal(high_fee_percent), status()]
}

fn transaction_screens(
    amount: &str,
    output_address: &str,
    total: &str,
    transaction_fee: &str,
    high_fee_percent: &str,
) -> Vec<Screen> {
    vec![
        address(amount, output_address),
        warning_fee(total, transaction_fee),
        high_fee_decimal(high_fee_percent),
        status(),
    ]
}

fn standard_expectations(
    prefix: Vec<Screen>,
    total: &str,
    transaction_fee: &str,
    high_fee_percent: u32,
) -> Vec<VersionExpectation> {
    let suffix_before_920 = vec![high_fee(high_fee_percent), status()];
    let suffix_920 = vec![
        address("0.20000000 TBTC", TBTC_EXTERNAL_ADDRESS),
        warning_fee(total, transaction_fee),
        high_fee(high_fee_percent),
        status(),
    ];
    let suffix_926 = vec![
        address("0.20000000 TBTC", TBTC_EXTERNAL_ADDRESS_GROUPED),
        warning_fee(total, transaction_fee),
        high_fee(high_fee_percent),
        status(),
    ];

    vec![
        success(
            None,
            Some("9.20.0"),
            with_suffix(&prefix, suffix_before_920),
        ),
        success(
            Some("9.20.0"),
            Some("9.26.0"),
            with_suffix(&prefix, suffix_920),
        ),
        success(Some("9.26.0"), None, with_suffix(&prefix, suffix_926)),
    ]
}

fn simple_expectations(
    middle: &[Screen],
    external_address: &str,
    grouped_external_address: &str,
    unit: &str,
) -> Vec<VersionExpectation> {
    let amount = format!("0.20000000 {unit}");
    let total = format!("0.30000000 {unit}");
    let fee_amount = format!("0.10000000 {unit}");
    let suffix = |address_value: &str| {
        let mut screens = vec![address(&amount, address_value)];
        screens.extend_from_slice(middle);
        screens.extend([warning_fee(&total, &fee_amount), high_fee(50), status()]);
        screens
    };
    let mut legacy = middle.to_vec();
    legacy.extend([high_fee(50), status()]);

    vec![
        success(None, Some("9.20.0"), legacy),
        success(Some("9.20.0"), Some("9.26.0"), suffix(external_address)),
        success(Some("9.26.0"), None, suffix(grouped_external_address)),
    ]
}

fn taproot_policy_expectations(prefix: Vec<Screen>) -> Vec<VersionExpectation> {
    let suffix_920 = vec![
        address("0.20000000 TBTC", TBTC_EXTERNAL_ADDRESS),
        warning_fee("0.30000000 TBTC", "0.10000000 TBTC"),
        high_fee(50),
        status(),
    ];
    let suffix_926 = vec![
        address("0.20000000 TBTC", TBTC_EXTERNAL_ADDRESS_GROUPED),
        warning_fee("0.30000000 TBTC", "0.10000000 TBTC"),
        high_fee(50),
        status(),
    ];

    vec![
        unsupported_before("9.21.0"),
        success(
            Some("9.21.0"),
            Some("9.26.0"),
            with_suffix(&prefix, suffix_920),
        ),
        success(Some("9.26.0"), None, with_suffix(&prefix, suffix_926)),
    ]
}

fn with_suffix(prefix: &[Screen], suffix: Vec<Screen>) -> Vec<Screen> {
    prefix.iter().cloned().chain(suffix).collect()
}

pub fn taproot_key_spend() -> Vec<VersionExpectation> {
    standard_expectations(vec![], "1.00000000 TBTC", "0.80000000 TBTC", 400)
}

pub fn simple_tbtc(middle: &[Screen]) -> Vec<VersionExpectation> {
    simple_expectations(
        middle,
        TBTC_EXTERNAL_ADDRESS,
        TBTC_EXTERNAL_ADDRESS_GROUPED,
        "TBTC",
    )
}

pub fn simple_ltc(middle: &[Screen]) -> Vec<VersionExpectation> {
    simple_expectations(
        middle,
        LTC_EXTERNAL_ADDRESS,
        LTC_EXTERNAL_ADDRESS_GROUPED,
        "LTC",
    )
}

pub fn multiple_output_types(
    coin: Coin,
    sat: bool,
    high_fee_warning: bool,
) -> Vec<VersionExpectation> {
    assert!(!high_fee_warning || coin == Coin::Btc && !sat);
    let (unit, addresses, grouped_addresses) = match coin {
        Coin::Btc => (
            "BTC",
            [
                "12ZEw5Hcv1hTb6YUQJ69y1V7uhcoDz92PH",
                "34oVnh4gNviJGMnNvgquMeLAxvXJuaRVMZ",
                "bc1qxvenxvenxvenxvenxvenxvenxvenxven2ymjt8",
                "bc1qg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zqd8sxw4",
            ],
            [
                "12ZE w5Hc v1hT b6YU QJ69 y1V7 uhco Dz92 PH",
                "34oV nh4g NviJ GMnN vgqu MeLA xvXJ uaRV MZ",
                "bc1q xven xven xven xven xven xven xven xven 2ymj t8",
                "bc1q g3zy g3zy g3zy g3zy g3zy g3zy g3zy g3zy g3zy g3zy g3zy g3zy g3zq d8sx w4",
            ],
        ),
        Coin::Ltc => (
            "LTC",
            [
                "LLnCCHbSzfwWquEdaS5TF2Yt7uz5Qb1SZ1",
                "MB1e6aUeL3Zj4s4H2ZqFBHaaHd7kvvzTco",
                "ltc1qxvenxvenxvenxvenxvenxvenxvenxvenwcpknh",
                "ltc1qg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zqwr7k5s",
            ],
            [
                "LLnC CHbS zfwW quEd aS5T F2Yt 7uz5 Qb1S Z1",
                "MB1e 6aUe L3Zj 4s4H 2ZqF BHaa Hd7k vvzT co",
                "ltc1 qxve nxve nxve nxve nxve nxve nxve nxve nwcp knh",
                "ltc1 qg3z yg3z yg3z yg3z yg3z yg3z yg3z yg3z yg3z yg3z yg3z yg3z yg3z qwr7 k5s",
            ],
        ),
        Coin::Tbtc => panic!("multiple-output fixture has no TBTC address set"),
    };
    let amounts = if sat {
        [
            "100000000 sat".into(),
            "1234567890 sat".into(),
            "6000 sat".into(),
            "7000 sat".into(),
        ]
    } else {
        [
            format!("1.00000000 {unit}"),
            format!(
                "{} {unit}",
                if high_fee_warning {
                    "10.34567890"
                } else {
                    "12.34567890"
                }
            ),
            format!("0.00006000 {unit}"),
            format!("0.00007000 {unit}"),
        ]
    };
    let change_warning = || Screen::Confirm {
        title: "Warning".into(),
        body: "There are 2\nchange outputs.\nProceed?".into(),
        longtouch: false,
    };
    let final_screens = |addresses: [&str; 4]| {
        let mut result = addresses
            .into_iter()
            .zip(&amounts)
            .map(|(output_address, amount)| address(amount, output_address))
            .collect::<Vec<_>>();
        result.push(change_warning());
        result.push(if sat {
            final_fee("1339999900 sat", "5419010 sat")
        } else if high_fee_warning {
            warning_fee("13.39999900 BTC", "2.05419010 BTC")
        } else {
            final_fee(
                &format!("13.39999900 {unit}"),
                &format!("0.05419010 {unit}"),
            )
        });
        if high_fee_warning {
            result.push(high_fee_decimal("18.1"));
        }
        result.push(status());
        result
    };
    let mut legacy_screens = vec![change_warning()];
    if high_fee_warning {
        legacy_screens.push(high_fee_decimal("18.1"));
    }
    legacy_screens.push(status());
    vec![
        success(None, Some("9.20.0"), legacy_screens),
        success(Some("9.20.0"), Some("9.26.0"), final_screens(addresses)),
        success(Some("9.26.0"), None, final_screens(grouped_addresses)),
    ]
}

pub fn p2tr_output_btc() -> Vec<VersionExpectation> {
    simple_expectations(&[], BTC_P2TR_ADDRESS, BTC_P2TR_ADDRESS_GROUPED, "BTC")
}

pub fn always_invalid_input() -> Vec<VersionExpectation> {
    vec![invalid_input(None, None)]
}

pub fn always_invalid_input_with_screens(screens: Vec<Screen>) -> Vec<VersionExpectation> {
    vec![invalid_input_with_screens(None, None, screens)]
}

pub fn unsupported_then_invalid_input(version: &str) -> Vec<VersionExpectation> {
    vec![
        unsupported_before(version),
        invalid_input(Some(version), None),
    ]
}

pub fn swap_payment_request() -> Vec<VersionExpectation> {
    vec![
        invalid_input(None, Some("9.24.0")),
        invalid_input_with_screens(
            Some("9.24.0"),
            Some("9.26.0"),
            vec![address("0.20000000 BTC", "Test Merchant")],
        ),
        success(
            Some("9.26.0"),
            None,
            vec![
                address("0.20000000 BTC", "Test Merchant"),
                Screen::Swap {
                    title: "Swap".into(),
                    from: "0.20000000 BTC".into(),
                    to: "0.25 ETH".into(),
                },
                warning_fee("0.30000000 BTC", "0.10000000 BTC"),
                high_fee(50),
                status(),
            ],
        ),
    ]
}

pub fn swap_payment_request_unsupported_source() -> Vec<VersionExpectation> {
    vec![
        invalid_input(None, Some("9.24.0")),
        invalid_input_with_screens(
            Some("9.24.0"),
            Some("9.26.0"),
            vec![address("0.20000000 TBTC", "Test Merchant")],
        ),
        invalid_input(Some("9.26.0"), None),
    ]
}

pub fn mixed_spend() -> Vec<VersionExpectation> {
    standard_expectations(vec![], "2.00000000 TBTC", "1.80000000 TBTC", 900)
}

pub fn op_return() -> Vec<VersionExpectation> {
    vec![
        unsupported_before("9.24.0"),
        success(
            Some("9.24.0"),
            None,
            vec![
                Screen::Confirm {
                    title: "OP_RETURN".into(),
                    body: "hello world".into(),
                    longtouch: false,
                },
                final_fee("0.01000000 TBTC", "0.01000000 TBTC"),
                status(),
            ],
        ),
    ]
}

pub fn op_return_nonascii() -> Vec<VersionExpectation> {
    let screens = |external_address: &str| {
        vec![
            Screen::Confirm {
                title: "OP_RETURN\ndata (hex)".into(),
                body: "0102030405".into(),
                longtouch: false,
            },
            address("0.20000000 TBTC", external_address),
            warning_fee("0.30000000 TBTC", "0.10000000 TBTC"),
            high_fee(50),
            status(),
        ]
    };
    vec![
        unsupported_before("9.24.0"),
        success(
            Some("9.24.0"),
            Some("9.26.0"),
            screens(TBTC_EXTERNAL_ADDRESS),
        ),
        success(Some("9.26.0"), None, screens(TBTC_EXTERNAL_ADDRESS_GROUPED)),
    ]
}

pub fn multisig(threshold: u32, xpub_count: usize, name: &str) -> Vec<VersionExpectation> {
    standard_expectations(
        vec![
            Screen::Confirm {
                title: "Spend from".into(),
                body: format!("{threshold}-of-{xpub_count}\nBTC Testnet multisig"),
                longtouch: false,
            },
            Screen::Confirm {
                title: "Spend from".into(),
                body: name.into(),
                longtouch: false,
            },
        ],
        "0.30000000 TBTC",
        "0.10000000 TBTC",
        50,
    )
}

pub fn multisig_p2wsh() -> Vec<VersionExpectation> {
    multisig(1, 2, "test wsh multisig")
}

pub fn policy_prefix(policy: &str, name: &str, keys: &[String]) -> Vec<Screen> {
    let mut result = vec![
        Screen::Confirm {
            title: "Spend from".into(),
            body: format!("BTC Testnet\npolicy with\n{} keys", keys.len()),
            longtouch: false,
        },
        Screen::Confirm {
            title: "Name".into(),
            body: name.into(),
            longtouch: false,
        },
        Screen::Confirm {
            title: "".into(),
            body: "Show policy\ndetails?".into(),
            longtouch: false,
        },
        Screen::Confirm {
            title: "Policy".into(),
            body: policy.into(),
            longtouch: false,
        },
    ];
    result.extend(keys.iter().enumerate().map(|(index, key)| Screen::Confirm {
        title: format!("Key {}/{}", index + 1, keys.len()),
        body: key.clone(),
        longtouch: false,
    }));
    result
}

pub fn policy(policy: &str, name: &str, keys: &[String], taproot: bool) -> Vec<VersionExpectation> {
    let prefix = policy_prefix(policy, name, keys);
    if taproot {
        taproot_policy_expectations(prefix)
    } else {
        standard_expectations(prefix, "0.30000000 TBTC", "0.10000000 TBTC", 50)
    }
}

pub fn policy_wsh() -> Vec<VersionExpectation> {
    policy(
        "wsh(or_b(pk(@0/<0;1>/*),s:pk(@1/<0;1>/*)))",
        "test wsh policy",
        &[
            format!("This device: {DEVICE_POLICY_XPUB}"),
            SOME_XPUB.into(),
        ],
        false,
    )
}

pub fn policy_tr_keyspend() -> Vec<VersionExpectation> {
    policy(
        "tr(@0/<0;1>/*)",
        "test tr keyspend policy",
        &[format!("This device: {DEVICE_POLICY_XPUB}")],
        true,
    )
}

pub fn policy_tr_scriptspend() -> Vec<VersionExpectation> {
    policy(
        "tr(@0/<0;1>/*,pk(@1/<0;1>/*))",
        "test tr scriptspend policy",
        &[
            SOME_XPUB.into(),
            format!("This device: {DEVICE_POLICY_XPUB}"),
        ],
        true,
    )
}

pub fn taproot_to_non_taproot_change() -> Vec<VersionExpectation> {
    standard_expectations(vec![], "1.00000000 TBTC", "0.80000000 TBTC", 400)
}

pub fn silent_payment() -> Vec<VersionExpectation> {
    vec![
        unsupported_before("9.21.0"),
        success(
            Some("9.21.0"),
            Some("9.26.0"),
            transaction_screens(
                "0.20000000 BTC",
                SILENT_PAYMENT_ADDRESS,
                "2.00000000 BTC",
                "1.80000000 BTC",
                "900.0",
            ),
        ),
        success(
            Some("9.26.0"),
            None,
            transaction_screens(
                "0.20000000 BTC",
                SILENT_PAYMENT_ADDRESS_GROUPED,
                "2.00000000 BTC",
                "1.80000000 BTC",
                "900.0",
            ),
        ),
    ]
}

pub fn send_self_same_account() -> Vec<VersionExpectation> {
    let old_address = format!("This BitBox02: {PSBT_SAME_ACCOUNT_ADDRESS}");
    let address = format!("This BitBox (same account): {PSBT_SAME_ACCOUNT_ADDRESS}");
    let grouped_address =
        format!("This BitBox (same account): {PSBT_SAME_ACCOUNT_ADDRESS_GROUPED}");

    vec![
        success(None, Some("9.20.0"), confirmed_screens("150.0")),
        success(
            Some("9.20.0"),
            Some("9.22.0"),
            transaction_screens(
                "0.20000000 TBTC",
                &old_address,
                "0.50000000 TBTC",
                "0.30000000 TBTC",
                "150.0",
            ),
        ),
        success(
            Some("9.22.0"),
            Some("9.26.0"),
            transaction_screens(
                "0.20000000 TBTC",
                &address,
                "0.50000000 TBTC",
                "0.30000000 TBTC",
                "150.0",
            ),
        ),
        success(
            Some("9.26.0"),
            None,
            transaction_screens(
                "0.20000000 TBTC",
                &grouped_address,
                "0.50000000 TBTC",
                "0.30000000 TBTC",
                "150.0",
            ),
        ),
    ]
}

pub fn silent_payment_owned_output() -> Vec<VersionExpectation> {
    let old_address = format!("This BitBox02: {SILENT_PAYMENT_ADDRESS}");
    let address = format!("This BitBox (same account): {SILENT_PAYMENT_ADDRESS}");
    let grouped_address = format!("This BitBox (same account): {SILENT_PAYMENT_ADDRESS_GROUPED}");

    vec![
        unsupported_before("9.21.0"),
        success(
            Some("9.21.0"),
            Some("9.22.0"),
            transaction_screens(
                "0.20000000 BTC",
                &old_address,
                "0.30000000 BTC",
                "0.10000000 BTC",
                "50.0",
            ),
        ),
        success(
            Some("9.22.0"),
            Some("9.26.0"),
            transaction_screens(
                "0.20000000 BTC",
                &address,
                "0.30000000 BTC",
                "0.10000000 BTC",
                "50.0",
            ),
        ),
        success(
            Some("9.26.0"),
            Some("9.26.3"),
            transaction_screens(
                "0.20000000 BTC",
                &grouped_address,
                "0.30000000 BTC",
                "0.10000000 BTC",
                "50.0",
            ),
        ),
        invalid_input(Some("9.26.3"), None),
    ]
}

pub fn payment_request_owned_output() -> Vec<VersionExpectation> {
    vec![
        invalid_input(None, Some("9.24.0")),
        success(
            Some("9.24.0"),
            Some("9.26.3"),
            payment_request_screens("0.30000000 TBTC", "0.10000000 TBTC", 50),
        ),
        invalid_input(Some("9.26.3"), None),
    ]
}

fn send_self_different_account_modern() -> Vec<VersionExpectation> {
    let address = format!("This BitBox (account #2): {PSBT_OTHER_ACCOUNT_ADDRESS}");
    let grouped_address = format!("This BitBox (account #2): {PSBT_OTHER_ACCOUNT_ADDRESS_GROUPED}");

    vec![
        success(
            Some("9.22.0"),
            Some("9.26.0"),
            transaction_screens(
                "0.20000000 TBTC",
                &address,
                "0.50000000 TBTC",
                "0.30000000 TBTC",
                "150.0",
            ),
        ),
        success(
            Some("9.26.0"),
            None,
            transaction_screens(
                "0.20000000 TBTC",
                &grouped_address,
                "0.50000000 TBTC",
                "0.30000000 TBTC",
                "150.0",
            ),
        ),
    ]
}

pub fn send_self_different_account() -> Vec<VersionExpectation> {
    vec![
        success(None, Some("9.20.0"), confirmed_screens("150.0")),
        success(
            Some("9.20.0"),
            Some("9.22.0"),
            transaction_screens(
                "0.20000000 TBTC",
                PSBT_OTHER_ACCOUNT_ADDRESS,
                "0.50000000 TBTC",
                "0.30000000 TBTC",
                "150.0",
            ),
        ),
    ]
    .into_iter()
    .chain(send_self_different_account_modern())
    .collect()
}

fn payment_request_screens(
    total: &str,
    transaction_fee: &str,
    high_fee_percent: u32,
) -> Vec<Screen> {
    vec![
        address("0.20000000 TBTC", "Test Merchant"),
        Screen::Confirm {
            title: "".into(),
            body: "Memo from\n\nTest Merchant".into(),
            longtouch: false,
        },
        Screen::Confirm {
            title: "Memo 1/2".into(),
            body: "TextMemo line1".into(),
            longtouch: false,
        },
        Screen::Confirm {
            title: "Memo 2/2".into(),
            body: "TextMemo line2".into(),
            longtouch: false,
        },
        warning_fee(total, transaction_fee),
        high_fee(high_fee_percent),
        status(),
    ]
}

pub fn payment_request() -> Vec<VersionExpectation> {
    vec![
        invalid_input_before("9.24.0"),
        success(
            Some("9.24.0"),
            None,
            payment_request_screens("0.50000000 TBTC", "0.30000000 TBTC", 150),
        ),
    ]
}
