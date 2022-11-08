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
	"encoding/binary"
	"errors"
	"fmt"
)

const (
	ConfigSize     = 128
	KeySize        = 32
	PrivateKeySize = 32
	PublicKeySize  = 64
	SignatureSize  = 64
)

type LockZone int

const (
	LockZoneConfig LockZone = 0
	LockZoneData            = 1
)

type Config struct {
	SerialNum    []byte
	Revision     uint32
	AESEnable    uint8
	I2CEnable    bool
	Reserved15   uint8
	I2CAddress   uint8
	Reserved17   uint8
	CountMatch   uint8
	ChipMode     ChipMode
	SlotInfo     [16]SlotInfo
	Counter0     uint64
	Counter1     uint64
	Todo0        uint64 // UseLock, VolatileKeyPermission, SecureBoot, KdfIvLoc, KdfIvStr, 1 bit reserved
	Todo1        uint64 // 8 bits reserved
	UserExtra    uint8
	UserExtraAdd uint8
	LockValue    LockMode
	LockConfig   LockMode
	// SlotLocked part of SlotInfo
	ChipOptions uint16
	X509Format  [4]X509Format
	// KeyConfig part of SlotInfo
}

// SetIOProtectionKeySlot defines the slot where the IO protection key is, and also sets the IO
// Protection Key Enable bit.
func (cfg *Config) SetIOProtectionKeySlot(slot uint16) {
	if slot > 15 {
		panic("slot must be between 0 and 15")
	}
	if slot != 0 {
		panic("TODO: verify if the endianness / bit order is correct")
	}
	// Set IO protection key slot.
	cfg.ChipOptions &= 0xFFF
	cfg.ChipOptions |= slot << 12
	// Enable IO protection key.
	cfg.ChipOptions |= 2
}

func (cfg *Config) EnableKDFProtection() {
	// Set ChipOptions bits 10-11 (KDF) to enforce output encryption.

	cfg.ChipOptions &= ^(uint16(1) << 11) // set bit to 0
	cfg.ChipOptions |= (1 << 10)          // set bit to 1
}

type ChipMode struct {
	I2CAddressUserExtraAddMode bool
	TTLEnable                  bool
	WatchDogDuration           WatchdogDuration
	ClockDivider               ClockDivider
}

type WatchdogDuration string

const (
	Watchdog1  WatchdogDuration = "1s"
	Watchdog13 WatchdogDuration = "13s"
)

type ClockDivider uint8

const (
	ClockDivider0  ClockDivider = 0
	ClockDivider0D ClockDivider = 0x0D
	ClockDivider05 ClockDivider = 0x05
)

// This struct is not stored in the chip and simply contains SlotLocked, SlotConfig and KeyConfig
//  together, for convenience and readability.
type SlotInfo struct {
	SlotLocked bool
	SlotConfig SlotConfig
	KeyConfig  KeyConfig
}

type SlotConfig struct {
	PrivateKeySlotConfig *PrivateKeySlotConfig `json:",omitempty" yaml:",omitempty"` // For slots 0-7
	ReadKey              *uint8                `json:",omitempty" yaml:",omitempty"` // For slots 8-15
	NoMAC                bool
	LimitedUse           bool
	EncryptRead          bool
	IsSecret             bool
	WriteKey             uint8
	WriteConfig          uint8
}

type PrivateKeySlotConfig struct {
	ExtSignEnable  bool
	IntSignEnable  bool
	ECDHEnable     bool
	ECDHToNextSlot bool
}

type LockMode string

const (
	LockModeLocked   LockMode = "Locked"
	LockModeUnlocked LockMode = "Unlocked"
)

type X509Format struct {
	PublicPosition uint8
	TemplateLength uint8
}

type KeyConfig struct {
	Private           bool    // 0
	PubInfo           bool    // 1
	KeyType           KeyType // 2, 3, 4
	Lockable          bool    // 5
	ReqRandom         bool    // 6
	ReqAuth           bool    // 7
	AuthKey           uint8   // 8,9,10,11
	PersistentDisable bool    // 12
	// 13 - Reserved
	X509ID uint8 // 14,15
}

type KeyType string

const (
	KeyTypeECC    KeyType = "ECC"
	KeyTypeNonECC         = "NonECC"
)

func ParseSlotConfig(scv uint16, kc *KeyConfig) SlotConfig {
	sc := SlotConfig{}
	if kc.Private {
		pkc := &PrivateKeySlotConfig{}
		pkc.ExtSignEnable = (scv&1 != 0)
		pkc.IntSignEnable = (scv&2 != 0)
		pkc.ECDHEnable = (scv&4 != 0)
		pkc.ECDHToNextSlot = (scv&8 != 0)
		sc.PrivateKeySlotConfig = pkc
	} else {
		rk := uint8(scv & 0xF)
		sc.ReadKey = &rk
	}
	sc.NoMAC = (scv&0x10 != 0)
	sc.LimitedUse = (scv&0x20 != 0)
	sc.EncryptRead = (scv&0x40 != 0)
	sc.IsSecret = (scv&0x80 != 0)
	sc.WriteKey = uint8((scv >> 8) & 0xF)
	sc.WriteConfig = uint8((scv >> 12) & 0xF)
	return sc
}

func parseLockMode(b uint8) (LockMode, error) {
	if b == 0x55 {
		return LockModeUnlocked, nil
	} else if b == 0x00 {
		return LockModeLocked, nil
	} else {
		return "", fmt.Errorf("unknown data lock mode 0x%02x", b)
	}
}

func parseKeyConfig(num int, kcv uint16) (*KeyConfig, error) {
	var err error
	kc := &KeyConfig{}
	kc.Private = (kcv&1 != 0)
	kc.PubInfo = (kcv&2 != 0)
	kc.KeyType, err = parseKeyType(uint8((kcv >> 2) & 0x7))
	if err != nil {
		return nil, err
	}
	kc.Lockable = (kcv&0x20 != 0)
	kc.ReqRandom = (kcv&0x40 != 0)
	kc.ReqAuth = (kcv&0x80 != 0)
	kc.AuthKey = uint8((kcv >> 8) & 0xF)
	kc.PersistentDisable = (kcv&0x1000 != 0)
	kc.X509ID = uint8((kcv >> 14) & 0x3)
	return kc, nil
}

func parseKeyType(b uint8) (KeyType, error) {
	if b == 4 {
		return KeyTypeECC, nil
	} else if b == 7 {
		return KeyTypeNonECC, nil
	} else {
		return "", fmt.Errorf("unknown key type %d", b)
	}
}

func ParseBinaryConfig(cd []byte) (*Config, error) {
	cb := bytes.NewBuffer(cd)
	var b uint8
	var err error
	if len(cd) != ConfigSize {
		return nil, fmt.Errorf("expected %d bytes, got %d", ConfigSize, len(cd))
	}
	c := &Config{}
	c.SerialNum = make([]byte, 9)
	cb.Read(c.SerialNum[0:4])
	binary.Read(cb, binary.BigEndian, &c.Revision)
	cb.Read(c.SerialNum[4:9])
	binary.Read(cb, binary.BigEndian, &c.AESEnable)
	binary.Read(cb, binary.BigEndian, &b)
	c.I2CEnable = (b&1 != 0)
	binary.Read(cb, binary.BigEndian, &c.Reserved15)
	binary.Read(cb, binary.BigEndian, &c.I2CAddress)
	binary.Read(cb, binary.BigEndian, &c.Reserved17)
	binary.Read(cb, binary.BigEndian, &c.CountMatch)
	binary.Read(cb, binary.BigEndian, &b)
	c.ChipMode.I2CAddressUserExtraAddMode = (b&1 != 0)
	c.ChipMode.TTLEnable = (b&2 != 0)
	if b&4 != 0 {
		c.ChipMode.WatchDogDuration = Watchdog13
	} else {
		c.ChipMode.WatchDogDuration = Watchdog1
	}
	c.ChipMode.ClockDivider = ClockDivider(b >> 3)
	switch c.ChipMode.ClockDivider {
	case ClockDivider0, ClockDivider0D, ClockDivider05:
	default:
		return nil, errors.New("invalid chipmode clockdivider")
	}
	var scvs [16]uint16
	for i := 0; i < 16; i++ {
		// We need to know slot's KeyConfig.Private setting to know how to parse SlotConfig.ReadKey.
		binary.Read(cb, binary.LittleEndian, &scvs[i])
	}
	binary.Read(cb, binary.BigEndian, &c.Counter0)
	binary.Read(cb, binary.BigEndian, &c.Counter1)
	binary.Read(cb, binary.BigEndian, &c.Todo0)
	binary.Read(cb, binary.BigEndian, &c.Todo1)
	binary.Read(cb, binary.BigEndian, &c.UserExtra)
	binary.Read(cb, binary.BigEndian, &c.UserExtraAdd)
	binary.Read(cb, binary.BigEndian, &b)
	c.LockValue, err = parseLockMode(b)
	if err != nil {
		return nil, err
	}
	binary.Read(cb, binary.BigEndian, &b)
	c.LockConfig, err = parseLockMode(b)
	if err != nil {
		return nil, err
	}
	var slotLocked uint16
	binary.Read(cb, binary.LittleEndian, &slotLocked)
	binary.Read(cb, binary.LittleEndian, &c.ChipOptions)
	for i := 0; i < 4; i++ {
		var fc X509Format
		binary.Read(cb, binary.BigEndian, &b)
		fc.PublicPosition = (b & 0xF)
		fc.TemplateLength = ((b >> 4) & 0xF)
		c.X509Format[i] = fc
	}
	for i := 0; i < 16; i++ {
		var kcv uint16
		binary.Read(cb, binary.LittleEndian, &kcv)
		kc, err := parseKeyConfig(i, kcv)
		if err != nil {
			return nil, err
		}
		c.SlotInfo[i] = SlotInfo{
			SlotLocked: (slotLocked & (1 << uint16(i))) == 0,
			SlotConfig: ParseSlotConfig(scvs[i], kc),
			KeyConfig:  *kc,
		}
	}
	return c, nil
}

func WriteSlotConfig(cb *bytes.Buffer, si SlotInfo) error {
	var scv uint16
	sc := &si.SlotConfig
	if si.KeyConfig.Private {
		pkc := sc.PrivateKeySlotConfig
		if pkc == nil {
			return fmt.Errorf("no PrivateKeyConfig")
		}
		if pkc.ExtSignEnable {
			scv |= 1
		}
		if pkc.IntSignEnable {
			scv |= 2
		}
		if pkc.ECDHEnable {
			scv |= 4
		}
		if pkc.ECDHToNextSlot {
			scv |= 8
		}
	} else {
		if sc.ReadKey == nil {
			return fmt.Errorf("no ReadKey")
		}
		scv = uint16(*sc.ReadKey)
	}
	if sc.NoMAC {
		scv |= 0x10
	}
	if sc.LimitedUse {
		scv |= 0x20
	}
	if sc.EncryptRead {
		scv |= 0x40
	}
	if sc.IsSecret {
		scv |= 0x80
	}
	scv |= (uint16(sc.WriteKey&0xF) << 8)
	scv |= (uint16(sc.WriteConfig&0xF) << 12)
	binary.Write(cb, binary.LittleEndian, scv)
	return nil
}

func WriteKeyConfig(cb *bytes.Buffer, si SlotInfo) error {
	var kcv uint16
	kc := &si.KeyConfig
	if kc.Private {
		kcv |= 1
	}
	if kc.PubInfo {
		kcv |= 2
	}
	switch kc.KeyType {
	case KeyTypeECC:
		kcv |= (uint16(4) << 2)
	case KeyTypeNonECC:
		kcv |= (uint16(7) << 2)
	default:
		return fmt.Errorf("unknown key type '%s'", kc.KeyType)
	}
	if kc.Lockable {
		kcv |= 0x20
	}
	if kc.ReqRandom {
		kcv |= 0x40
	}
	if kc.ReqAuth {
		kcv |= 0x80
	}
	kcv |= (uint16(kc.AuthKey&0xF) << 8)
	if kc.PersistentDisable {
		kcv |= 0x1000
	}
	kcv |= (uint16(kc.X509ID&0x3) << 14)
	binary.Write(cb, binary.LittleEndian, kcv)
	return nil
}

func writeLockMode(cb *bytes.Buffer, lm LockMode) error {
	var b uint8
	switch lm {
	case LockModeUnlocked:
		b = 0x55
	case LockModeLocked:
		b = 0
	default:
		return fmt.Errorf("unknown lock mode %s", lm)
	}
	return binary.Write(cb, binary.BigEndian, b)
}

func WriteBinaryConfig(c *Config) ([]byte, error) {
	var b uint8
	var err error
	cd := make([]byte, 0, ConfigSize)
	cb := bytes.NewBuffer(cd)
	sn := c.SerialNum
	if sn == nil {
		sn = make([]byte, 9)
	}
	cb.Write(sn[0:4])
	binary.Write(cb, binary.BigEndian, c.Revision)
	cb.Write(sn[4:9])
	binary.Write(cb, binary.BigEndian, c.AESEnable)
	b = 0
	if c.I2CEnable {
		b |= 1
	}
	binary.Write(cb, binary.BigEndian, b)
	binary.Write(cb, binary.BigEndian, c.Reserved15)
	binary.Write(cb, binary.BigEndian, c.I2CAddress)
	binary.Write(cb, binary.BigEndian, c.Reserved17)
	binary.Write(cb, binary.BigEndian, c.CountMatch)
	b = 0
	if c.ChipMode.I2CAddressUserExtraAddMode {
		b |= 1
	}
	if c.ChipMode.TTLEnable {
		b |= 2
	}
	b |= uint8(c.ChipMode.ClockDivider) << 3
	switch c.ChipMode.WatchDogDuration {
	case Watchdog1:
		break
	case Watchdog13:
		b |= 4
	default:
		return nil, fmt.Errorf("unknown watchdog duration %s", c.ChipMode.WatchDogDuration)
	}
	binary.Write(cb, binary.BigEndian, b)
	for i := 0; i < 16; i++ {
		err = WriteSlotConfig(cb, c.SlotInfo[i])
		if err != nil {
			return nil, err
		}
	}
	binary.Write(cb, binary.BigEndian, c.Counter0)
	binary.Write(cb, binary.BigEndian, c.Counter1)
	binary.Write(cb, binary.BigEndian, c.Todo0)
	binary.Write(cb, binary.BigEndian, c.Todo1)
	binary.Write(cb, binary.BigEndian, c.UserExtra)
	binary.Write(cb, binary.BigEndian, c.UserExtraAdd)
	err = writeLockMode(cb, c.LockValue)
	if err != nil {
		return nil, err
	}
	err = writeLockMode(cb, c.LockConfig)
	if err != nil {
		return nil, err
	}
	var slotLocked uint16
	for i, slotInfo := range c.SlotInfo {
		if slotInfo.SlotLocked {
			slotLocked |= 1 << uint16(i)
		}
	}
	binary.Write(cb, binary.LittleEndian, ^slotLocked)
	binary.Write(cb, binary.LittleEndian, c.ChipOptions)
	b = 0
	for i := 0; i < 4; i++ {
		fc := &c.X509Format[i]
		b = 0
		b |= (fc.PublicPosition & 0xF)
		b |= ((fc.TemplateLength & 0xF) << 4)
		binary.Write(cb, binary.BigEndian, b)
	}
	for i := 0; i < 16; i++ {
		err = WriteKeyConfig(cb, c.SlotInfo[i])
		if err != nil {
			return nil, err
		}
	}
	return cb.Bytes(), nil
}
