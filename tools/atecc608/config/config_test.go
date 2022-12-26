// Copied and adapted from https://github.com/cesanta/mos-tool/tree/837916dcb506d5cd2adbff02f2cd06ea4ea8c71a/mos/atca
// https://github.com/cesanta/mos-tool/blob/837916dcb506d5cd2adbff02f2cd06ea4ea8c71a/mos/LICENSE
// Copyright (c) 2014-2018 Cesanta Software Limited
// Copyright 2019 Shift Cryptosecurity AG
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

package config

import (
	"bytes"
	"encoding/hex"
	"math"
	"testing"
)

var (
	testConfigGolden = [128]byte{
		0x01, 0x23, 0x52, 0xaa,
		0x00, 0x00, 0x50, 0x00,
		0xd1, 0xbb, 0xf3, 0x78,
		0xee, 0xc0, 0x01, 0x00,
		0xc0, 0x00, 0x55, 0x00,
		0x87, 0x64, 0x87, 0x64,
		0x87, 0x64, 0x87, 0x64,
		0x80, 0x0f, 0x8f, 0x8f,
		0x9f, 0x8f, 0x82, 0x20,
		0xc4, 0x44, 0xc4, 0x44,
		0x0f, 0x0f, 0x0f, 0x0f,
		0x0f, 0x0f, 0x0f, 0x0f,
		0x0f, 0x0f, 0x0f, 0x0f,
		0xff, 0xff, 0xff, 0xff,
		0x00, 0x00, 0x00, 0x00,
		0xff, 0xff, 0xff, 0xff,
		0x00, 0x00, 0x00, 0x00,
		0xff, 0xff, 0xff, 0xff,
		0xff, 0xff, 0xff, 0xff,
		0xff, 0xff, 0xff, 0xff,
		0xff, 0xff, 0xff, 0xff,
		0x00, 0x00, 0x00, 0x00,
		0xff, 0xff, 0x00, 0x00,
		0x00, 0x00, 0x00, 0x00,
		0x33, 0x00, 0x33, 0x00,
		0x33, 0x00, 0x33, 0x00,
		0x3c, 0x00, 0x1c, 0x00,
		0x1c, 0x00, 0x33, 0x00,
		0x1c, 0x00, 0x1c, 0x00,
		0x3c, 0x00, 0x3c, 0x00,
		0x3c, 0x00, 0x3c, 0x00,
		0x1c, 0x00, 0x3c, 0x00,
	}
)

func TestParseAndWrite(t *testing.T) {
	c, err := ParseBinaryConfig(testConfigGolden[:])
	if err != nil {
		t.Fatalf("failed to parse golden config: %s", err)
	}
	cb, err := WriteBinaryConfig(c)
	if err != nil {
		t.Fatalf("failed to serialize golden config: %s", err)
	}
	if bytes.Compare(cb, testConfigGolden[:]) != 0 {
		t.Fatalf("serialized config does not match golden:\nExpected: %s\nGot     : %s",
			hex.EncodeToString(testConfigGolden[:]), hex.EncodeToString(cb))
	}
}

func TestParseAndWriteClockDivider(t *testing.T) {
	for _, clockDivider := range []ClockDivider{
		ClockDivider0, ClockDivider0D, ClockDivider05,
	} {
		c, _ := ParseBinaryConfig(testConfigGolden[:])
		c.ChipMode.ClockDivider = clockDivider
		cb, err := WriteBinaryConfig(c)
		if err != nil {
			t.Fatal(err)
		}
		c2, err := ParseBinaryConfig(cb)
		if err != nil {
			t.Fatal(err)
		}
		if c.ChipMode.ClockDivider != c2.ChipMode.ClockDivider {
			t.Fatal("wrong clock divider")
		}
	}
}

func TestParseAndWriteSlotLocked(t *testing.T) {
	// Exhaustive test of all slotlocked possibilities
	for i := uint16(0); i < math.MaxUint16; i++ {
		c, _ := ParseBinaryConfig(testConfigGolden[:])
		for j := 0; j < 16; j++ {
			c.SlotInfo[j].SlotLocked = i&(1<<uint16(j)) == 0
		}
		cb, err := WriteBinaryConfig(c)
		if err != nil {
			t.Fatal(err)
		}
		if cb[88] != uint8(i) || cb[89] != uint8(i>>8) {
			t.Fatal("wrong serialization")
		}
		c2, err := ParseBinaryConfig(cb)
		if err != nil {
			t.Fatal(err)
		}
		for j := 0; j < 16; j++ {
			if c.SlotInfo[j].SlotLocked != c2.SlotInfo[j].SlotLocked {
				t.Fatalf("slotLocked did not match for %d", i)
			}
		}
	}
}
