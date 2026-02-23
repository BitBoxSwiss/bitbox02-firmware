// Generates typed_msg_streaming_tests.json using @metamask/eth-sig-util v4.0.1 and ethers v5.
//
// Usage:
//   npm install @metamask/eth-sig-util@v4.0.1 ethers@5
//   node gen_typed_msg_streaming_tests.js > typed_msg_streaming_tests.json

const util = require("@metamask/eth-sig-util");
const { ethers } = require("ethers");

const mnemonic =
  "purity concert above invest pigeon category peace tuition hazard vivid latin since legal speak nation session onion library travel spell region blast estate stay";
const wallet = ethers.Wallet.fromMnemonic(mnemonic, "m/44'/60'/0'/0/0");
const privateKey = Buffer.from(wallet.privateKey.slice(2), "hex");

function generateBytes(length, pattern) {
  const buf = Buffer.alloc(length);
  for (let i = 0; i < length; i++) {
    buf[i] = pattern(i);
  }
  return "0x" + buf.toString("hex");
}

const testCases = [
  {
    description: "large dynamic bytes",
    types: {
      EIP712Domain: [{ name: "name", type: "string" }],
      Msg: [{ name: "data", type: "bytes" }],
    },
    primaryType: "Msg",
    domain: { name: "test" },
    message: {
      data: generateBytes(10000, (i) => i % 256),
    },
    sign: true,
  },
  {
    description: "large string",
    types: {
      EIP712Domain: [{ name: "name", type: "string" }],
      Msg: [{ name: "data", type: "string" }],
    },
    primaryType: "Msg",
    domain: { name: "test" },
    message: {
      data: "a".repeat(10000),
    },
    sign: false,
  },
];

const output = testCases.map((tc) => {
  const msgParams = {
    types: tc.types,
    primaryType: tc.primaryType,
    domain: tc.domain,
    message: tc.message,
  };
  const sighash = util.TypedDataUtils.eip712Hash(msgParams, "V4");

  // For bytes fields, store hex-encoded value (without 0x prefix).
  // For string fields, store the raw string.
  let messageValue = tc.message.data;
  const fieldType = tc.types[tc.primaryType].find(
    (m) => m.name === "data"
  ).type;
  if (fieldType === "bytes") {
    // Strip 0x prefix for the JSON output.
    messageValue = messageValue.slice(2);
  }

  const result = {
    description: tc.description,
    types: tc.types,
    primary_type: tc.primaryType,
    domain: tc.domain,
    field_type: fieldType,
    message_data: messageValue,
    expected_sighash: sighash.toString("hex"),
  };

  if (tc.sign) {
    const sig = util.signTypedData({
      privateKey,
      data: msgParams,
      version: "V4",
    });
    // sig is hex string "0x" + r (32 bytes) + s (32 bytes) + v (1 byte), where v is 27 or 28.
    const sigBytes = Buffer.from(sig.slice(2), "hex");
    const r = sigBytes.slice(0, 32);
    const s = sigBytes.slice(32, 64);
    const v = sigBytes[64];
    // Firmware format: r + s + recid, where recid = v - 27.
    const firmwareSig = Buffer.concat([r, s, Buffer.from([v - 27])]);
    result.expected_signature = firmwareSig.toString("hex");
    result.address = wallet.address;
  }

  return result;
});

console.log(JSON.stringify(output, null, 2));
