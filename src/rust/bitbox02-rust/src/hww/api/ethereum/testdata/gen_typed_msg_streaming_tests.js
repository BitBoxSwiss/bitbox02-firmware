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

// Mirrors firmware's address formatting: "0x" prefix separated, rest in groups of 4.
function formatEthAddress(address) {
  const rest = address.slice(2);
  return "0x " + rest.match(/.{1,4}/g).join(" ");
}

function generateBytes(length, pattern) {
  const buf = Buffer.alloc(length);
  for (let i = 0; i < length; i++) {
    buf[i] = pattern(i);
  }
  return "0x" + buf.toString("hex");
}

function formatDisplayLinePrefix(displayPath, lineNum, numLines) {
  if (numLines > 1) {
    return `${displayPath}, line ${lineNum + 1}/${numLines}`;
  }
  return displayPath;
}

function formatDisplayLineBody(displayPath, lineNum, numLines, line) {
  return `${formatDisplayLinePrefix(displayPath, lineNum, numLines)}: ${line}`;
}

function streamingDisplayByteCap(displayPath, dataLength) {
  const MAX_DISPLAY_SIZE = 640;
  const bodyLenWithoutHex =
    formatDisplayLinePrefix(displayPath, 0, 1).length + ": ".length + "0x".length;
  const visibleBytes = Math.floor(
    Math.max(0, MAX_DISPLAY_SIZE - bodyLenWithoutHex) / 2
  );
  return Math.min(dataLength, visibleBytes + Number(dataLength > visibleBytes));
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
    description: "multiline string with long second line",
    types: {
      EIP712Domain: [{ name: "name", type: "string" }],
      Msg: [{ name: "data", type: "string" }],
    },
    primaryType: "Msg",
    domain: { name: "test" },
    message: {
      data: `ok\n${"b".repeat(640)}`,
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

  const fieldType = tc.types[tc.primaryType].find(
    (m) => m.name === "data"
  ).type;
  const messageValue =
    fieldType === "bytes" ? tc.message.data.slice(2) : tc.message.data;

  // Compute expected screens shown by the firmware.
  const expectedScreens = [];
  const domainType = tc.types.EIP712Domain;
  for (let i = 0; i < domainType.length; i++) {
    const field = domainType[i];
    expectedScreens.push([
      `Domain (${i + 1}/${domainType.length})`,
      `${field.name}: ${tc.domain[field.name]}`,
    ]);
  }
  const MAX_DISPLAY_SIZE = 640;
  const msgType = tc.types[tc.primaryType];
  for (let i = 0; i < msgType.length; i++) {
    const field = msgType[i];
    const title = `Message (${i + 1}/${msgType.length})`;
    let valueFormatted;
    if (field.type === "bytes") {
      const rawBytes = Buffer.from(tc.message[field.name].slice(2), "hex");
      const displayCap = streamingDisplayByteCap(field.name, rawBytes.length);
      const truncated = rawBytes.slice(0, displayCap);
      valueFormatted = "0x" + truncated.toString("hex");
    } else if (field.type === "string") {
      valueFormatted = tc.message[field.name];
    } else {
      throw new Error(`Unhandled field type in test case: ${field.type}`);
    }
    const lines = valueFormatted.split("\n");
    for (let lineIdx = 0; lineIdx < lines.length; lineIdx++) {
      const body = formatDisplayLineBody(
        field.name,
        lineIdx,
        lines.length,
        lines[lineIdx]
      );
      if (body.length > MAX_DISPLAY_SIZE) {
        expectedScreens.push([
          "Warning",
          "The next value is\ntoo large to display\nin full",
        ]);
      }
      expectedScreens.push([title, body]);
    }
  }

  const result = {
    description: tc.description,
    types: tc.types,
    primary_type: tc.primaryType,
    domain: tc.domain,
    field_type: fieldType,
    message_data: messageValue,
    expected_sighash: sighash.toString("hex"),
    expected_screens: expectedScreens,
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
    result.address = formatEthAddress(wallet.address);
  }

  return result;
});

console.log(JSON.stringify(output, null, 2));
