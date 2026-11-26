// SPDX-License-Identifier: Apache-2.0

fn print(parts: &[&str]) {
    // Preserve C stdio buffering without copying secrets into a formatting buffer.
    for part in parts {
        crate::print_stdout(part);
    }
}

pub fn enter_string(title: &str) {
    print(&["ENTER SCREEN START\nTITLE: ", title, "\nENTER SCREEN END\n"]);
}

pub fn confirm(title: &str, body: &str) {
    print(&[
        "CONFIRM SCREEN START\nTITLE: ",
        title,
        "\nBODY: ",
        body,
        "\nCONFIRM SCREEN END\n",
    ]);
}

pub fn status(title: &str) {
    print(&[
        "STATUS SCREEN START\nTITLE: ",
        title,
        "\nSTATUS SCREEN END\n",
    ]);
}

pub fn transaction_address(amount: &str, address: &str) {
    print(&[
        "CONFIRM TRANSACTION ADDRESS SCREEN START\nAMOUNT: ",
        amount,
        "\nADDRESS: ",
        address,
        "\nCONFIRM TRANSACTION ADDRESS SCREEN END\n",
    ]);
}

pub fn swap(title: &str, from: &str, to: &str) {
    print(&[
        "CONFIRM SWAP SCREEN START\nTITLE: ",
        title,
        "\nFROM: ",
        from,
        "\nTO: ",
        to,
        "\nCONFIRM SWAP SCREEN END\n",
    ]);
}

pub fn transaction_fee(amount: &str, fee: &str) {
    print(&[
        "CONFIRM TRANSACTION FEE SCREEN START\nAMOUNT: ",
        amount,
        "\nFEE: ",
        fee,
        "\nCONFIRM TRANSACTION FEE SCREEN END\n",
    ]);
}
