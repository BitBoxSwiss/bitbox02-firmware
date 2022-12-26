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

package main

import (
	"encoding/hex"
	"fmt"

	"atecc608a/config"
)

const (
	// See securechip.h/securechip.c for how the slots are used.

	// ioProtectionKeySlot holds the io protection key and is referenced by the kdf slots for output
	// encryption. Use needs to be authorized using authKeySlot for encrypted reads/writes.
	ioProtectionKeySlot = 0

	// authKeySlot holds the authorization key and is referenced by other slots to require
	// authorization before use.
	authKeySlot = 1

	// encryptionKeySlot holds the key referenced as the WriteKey/ReadKey of slots allowing
	// encrypted reads or writes.
	encryptionKeySlot = 2

	// rollkeySlot is a DeriveKey/Roll slot. read/write disabled. Used to perform KDF. key rolling
	// happens at factory setup and with each device reset. monotonic counter attached.
	rollkeySlot = 3

	// kdfKeySlot is a write-only slot. 32 random bytes are written to it at factory setup and with
	// each device reset.
	kdfKeySlot = 4

	// attestationKeySlot is an ECC slot. read/write disabled. Key internally generated at factory
	// setup and used to sign the device attestation host challenge.
	attestationKeySlot = 5

	// eccUnsafeSignKeySlot is a ECC slot. read disabled, encrypted write enabled. Can be used to
	// write any secret key in order to use the chip to create NIST P256 signatures. This is not
	// meant to use the SC for security, but used as an alternative to adding firmware code for
	// signing with this curve.
	eccUnsafeSignKeySlot = 6

	// internalECCKeySlot is an ECC slot. read/write disabled. Key internally generated using
	// GenKey.
	internalECCKeySlot = 7

	// dataKeySlot is the chip's designated data slot with 416 bytes of space. encrypted read and
	// encrytped write enabled.
	dataKeySlot = 8

	// All pubkey/certificate slots (9-15) have the same config as dataKeySlot.
)

// defaultConfigurationHex is a working start configuration. The final configuration is created by
// parsing this and overwriting specific fields, like the slot and key configs.
const defaultConfigurationHex = `012368ee000060028a1dde66ee012900c000000080a780ac802cc48f8f8f8f8f9f8faf8f0000000000000000000000000000af8f01ffffff00000000ffffffff000000000000000000000000000000000000000000000000ffff0000000000003d007d003d007d001c001c001c001c003c003c003c003c003c003c003c001c00`

// createConfiguration contains code to modify the default configuration, e.g. setting the slot
// configs, etc.
func createConfiguration(cfg *config.Config) {
	if cfg.ChipOptions != 0 {
		panic("ChipOptions must be initialized to 0")
	}

	encryptionKeySlotVal := uint8(encryptionKeySlot)

	// empty readKey where a readKey does not apply (where read is disabled).
	var noReadKey uint8

	cfg.SetIOProtectionKeySlot(ioProtectionKeySlot)
	cfg.EnableKDFProtection()

	cfg.SlotInfo[ioProtectionKeySlot] = config.SlotInfo{
		// When data zone is locked, this slot is locked and cannot be modified under any
		// circumstances. The key needs to be written to it before the data zone us locked.
		SlotLocked: false,
		SlotConfig: config.SlotConfig{
			PrivateKeySlotConfig: nil,
			ReadKey:              &noReadKey,
			NoMAC:                false,
			LimitedUse:           false,
			EncryptRead:          false,
			IsSecret:             true,
			WriteKey:             0,
			// WritConfig = 0b1000 = can never be written or read after the data zone is locked.
			// Redundant with setting SlotLocked to true.
			WriteConfig: 0x8,
		},
		KeyConfig: config.KeyConfig{
			Private: false,
			PubInfo: false,
			// KeyTypeECC = 4, P256 NIST ECC private or public key
			// KeyTypeNonECC = 7, any other key (e.g. source for DeriveKey or KDF).
			KeyType: config.KeyTypeNonECC,
			// Slot lockable after data zone is locked, otherwise we can't write an initial value.
			Lockable:          true, // lock individually after writing the initial value
			ReqRandom:         true, // if true, Nonce must use Rand, not passthrough.
			ReqAuth:           false,
			AuthKey:           0, // must be zero if ReqAuth is false
			PersistentDisable: false,
			X509ID:            0,
		},
	}

	// AuthKey slot config is the same as the IO protection key slot config. Both are written once
	// and then locked.
	cfg.SlotInfo[authKeySlot] = cfg.SlotInfo[ioProtectionKeySlot]

	// encryptionKeySlot is the same as the IO protection key slot config, except for the
	// ReqAuth/AuthKey fields. It is also written once and then locked.
	cfg.SlotInfo[encryptionKeySlot] = config.SlotInfo{
		// When data zone is locked, this slot is locked and cannot be modified under any
		// circumstances. The key needs to be written to it before the data zone us locked.
		SlotLocked: false,
		SlotConfig: config.SlotConfig{
			PrivateKeySlotConfig: nil,
			ReadKey:              &noReadKey,
			NoMAC:                false,
			LimitedUse:           false,
			EncryptRead:          false,
			IsSecret:             true,
			WriteKey:             0,
			// WritConfig = 0b1000 = can never be written or read after the data zone is locked.
			// Redundant with setting SlotLocked to true.
			WriteConfig: 0x8,
		},
		KeyConfig: config.KeyConfig{
			Private: false,
			PubInfo: false,
			// KeyTypeECC = 4, P256 NIST ECC private or public key
			// KeyTypeNonECC = 7, any other key (e.g. source for DeriveKey or KDF).
			KeyType: config.KeyTypeNonECC,
			// Slot lockable after data zone is locked, otherwise we can't write an initial value.
			Lockable:  true, // lock individually after writing the initial value
			ReqRandom: true, // if true, Nonce must use Rand, not passthrough.

			// encrypted reads/write operations use those auth flags.
			ReqAuth: true,
			AuthKey: authKeySlot,

			PersistentDisable: false,
			X509ID:            0,
		},
	}

	cfg.SlotInfo[rollkeySlot] = config.SlotInfo{
		SlotLocked: false,
		SlotConfig: config.SlotConfig{
			PrivateKeySlotConfig: nil,
			ReadKey:              &noReadKey,
			NoMAC:                false,
			LimitedUse:           true,
			EncryptRead:          false,
			IsSecret:             true,
			// Unused in key roll mode
			WriteKey: 0,
			// GenKey: 0bXX1X (Private must be true, slot must be unlocked (SlotLocked bit 1))
			// DeriveKey: 0bAX1D (Private must be false, slot must be unlocked (SlotLocked bit 1))
			// A=0 => does not require MAC
			// A=1 => requires MAC
			// D=0 => Roll (source slot = target slot).
			// D=1 => Create (source slot = target slot's writekey)
			WriteConfig: 0x2, // = 0b0010, DeriveKey Roll, no MAC, Write never permitted.
		},
		KeyConfig: config.KeyConfig{
			Private: false,
			PubInfo: false,
			// KeyTypeECC = 4, P256 NIST ECC private or public key
			// KeyTypeNonECC = 7, any other key (e.g. source for DeriveKey or KDF).
			KeyType:  config.KeyTypeNonECC,
			Lockable: false, // rolling keys, so the slot cannot be locked.
			// If true, Nonce must use Rand, not passthrough.
			ReqRandom: true,
			ReqAuth:   true,
			// Must be zero if ReqAuth is false, or point to the auth key slot if true.
			AuthKey:           authKeySlot,
			PersistentDisable: false,
			X509ID:            0,
		},
	}

	cfg.SlotInfo[kdfKeySlot] = config.SlotInfo{
		SlotLocked: false,
		SlotConfig: config.SlotConfig{
			PrivateKeySlotConfig: nil,
			ReadKey:              &noReadKey,
			NoMAC:                false,
			LimitedUse:           false,
			// IsSecret = 1, EncryptedRead = 0: Reads never permitted.
			EncryptRead: false,
			IsSecret:    true,
			WriteKey:    encryptionKeySlot,
			// = 0b0100 => encrypted write, no derivekey/genkey. Actually 0bX10X, first and last bit
			// are ignored in this configuration.
			WriteConfig: 0x4,
		},
		KeyConfig: config.KeyConfig{
			Private: false,
			PubInfo: false,
			// KeyTypeECC = 4, P256 NIST ECC private or public key
			// KeyTypeNonECC = 7, any other key (e.g. source for DeriveKey or KDF).
			KeyType:  config.KeyTypeNonECC,
			Lockable: false,
			// If true, Nonce must use Rand, not passthrough.
			ReqRandom: true,
			ReqAuth:   true,
			// Must be zero if ReqAuth is false, or point to the auth key slot if true.
			AuthKey:           authKeySlot,
			PersistentDisable: false,
			X509ID:            0,
		},
	}

	cfg.SlotInfo[attestationKeySlot] = config.SlotInfo{
		SlotLocked: false,
		SlotConfig: config.SlotConfig{
			PrivateKeySlotConfig: &config.PrivateKeySlotConfig{
				ExtSignEnable:  true,
				IntSignEnable:  true,
				ECDHEnable:     false,
				ECDHToNextSlot: false,
			},
			ReadKey:    &noReadKey,
			NoMAC:      false,
			LimitedUse: false,
			// IsSecret = 1, EncryptedRead = 0: Reads never permitted.
			EncryptRead: false,
			IsSecret:    true,
			// Reuse io protection key for encrypted writes.
			WriteKey: 0,
			// = 0b0010 => PrivWrite forbidden, GenKey enabled. Actually 0bX01X, first and last bit
			// are ignored in this configuration.
			WriteConfig: 0x2,
		},
		KeyConfig: config.KeyConfig{
			Private: true,
			PubInfo: true,
			// KeyTypeECC = 4, P256 NIST ECC private or public key
			// KeyTypeNonECC = 7, any other key (e.g. source for DeriveKey or KDF).
			KeyType:  config.KeyTypeECC,
			Lockable: true,
			// If true, Nonce must use Rand, not passthrough.
			ReqRandom: true,
			ReqAuth:   true,
			// Must be zero if ReqAuth is false, or point to the auth key slot if true.
			AuthKey:           authKeySlot,
			PersistentDisable: false,
			X509ID:            0,
		},
	}

	cfg.SlotInfo[eccUnsafeSignKeySlot] = config.SlotInfo{
		SlotLocked: false,
		SlotConfig: config.SlotConfig{
			PrivateKeySlotConfig: &config.PrivateKeySlotConfig{
				ExtSignEnable:  true,
				IntSignEnable:  true,
				ECDHEnable:     false,
				ECDHToNextSlot: false,
			},
			ReadKey:    &noReadKey,
			NoMAC:      false,
			LimitedUse: false,
			// IsSecret = 1, EncryptedRead = 0: Reads never permitted.
			EncryptRead: false,
			IsSecret:    true,
			// Reuse io protection key for encrypted writes.
			WriteKey: encryptionKeySlot,
			// = 0b0110 => PrivWrite enabled, GenKey enabled. Actually 0bX11X, first and last bit
			// are ignored in this configuration.
			WriteConfig: 0x4 | 0x2,
		},
		KeyConfig: config.KeyConfig{
			Private: true,
			PubInfo: true,
			// KeyTypeECC = 4, P256 NIST ECC private or public key
			// KeyTypeNonECC = 7, any other key (e.g. source for DeriveKey or KDF).
			KeyType: config.KeyTypeECC,
			// Not lockable as we want to be able to write new secret keys at any time.
			Lockable: false,
			// If true, Nonce must use Rand, not passthrough.
			ReqRandom: true,
			ReqAuth:   true,
			// Must be zero if ReqAuth is false, or point to the auth key slot if true.
			AuthKey:           authKeySlot,
			PersistentDisable: false,
			X509ID:            0,
		},
	}

	// same as attestationKeySlot, but with Lockable=false.
	cfg.SlotInfo[internalECCKeySlot] = config.SlotInfo{
		SlotLocked: false,
		SlotConfig: config.SlotConfig{
			PrivateKeySlotConfig: &config.PrivateKeySlotConfig{
				ExtSignEnable:  true,
				IntSignEnable:  true,
				ECDHEnable:     false,
				ECDHToNextSlot: false,
			},
			ReadKey:    &noReadKey,
			NoMAC:      false,
			LimitedUse: false,
			// IsSecret = 1, EncryptedRead = 0: Reads never permitted.
			EncryptRead: false,
			IsSecret:    true,
			WriteKey:    0,
			// = 0b0010 => PrivWrite forbidden, GenKey enabled. Actually 0bX01X, first and last bit
			// are ignored in this configuration.
			WriteConfig: 0x2,
		},
		KeyConfig: config.KeyConfig{
			Private: true,
			PubInfo: true,
			// KeyTypeECC = 4, P256 NIST ECC private or public key
			// KeyTypeNonECC = 7, any other key (e.g. source for DeriveKey or KDF).
			KeyType:  config.KeyTypeECC,
			Lockable: false,
			// If true, Nonce must use Rand, not passthrough.
			ReqRandom: true,
			ReqAuth:   true,
			// Must be zero if ReqAuth is false, or point to the auth key slot if true.
			AuthKey:           authKeySlot,
			PersistentDisable: false,
			X509ID:            0,
		},
	}

	// Configure remaining slots (data slot 8 and pubkey slots 9-15) to be simple data storage
	// (encrypted read/write, no other functionality).
	for keySlotIndex := dataKeySlot; keySlotIndex < 16; keySlotIndex++ {
		cfg.SlotInfo[keySlotIndex] = config.SlotInfo{
			SlotLocked: false,
			SlotConfig: config.SlotConfig{
				PrivateKeySlotConfig: nil,
				// Reuse io protection key for encrypted reads.
				ReadKey:     &encryptionKeySlotVal,
				NoMAC:       false,
				LimitedUse:  false,
				EncryptRead: true,
				IsSecret:    true,
				// Reuse io protection key for encrypted writes.
				WriteKey: encryptionKeySlot,
				// = 0b0100 => encrypted write, no derivekey/genkey. Actually 0bX10X, first and last bit
				// are ignored in this configuration.
				WriteConfig: 0x4,
			},
			KeyConfig: config.KeyConfig{
				Private: false,
				PubInfo: false,
				// KeyTypeECC = 4, P256 NIST ECC private or public key
				// KeyTypeNonECC = 7, any other key (e.g. source for DeriveKey or KDF).
				KeyType:  config.KeyTypeNonECC,
				Lockable: false,
				// If true, Nonce must use Rand, not passthrough.
				ReqRandom: true,

				// Actually ignored, the corresponding fields of WriteKey are used when doing
				// encrypted reads/writes, which are the only operations enabled.
				ReqAuth: true,
				// Must be zero if ReqAuth is false, or point to the auth key slot if true.
				AuthKey: authKeySlot,

				PersistentDisable: false,
				X509ID:            0,
			},
		}
	}
}

func main() {
	defaultConfiguration, err := hex.DecodeString(defaultConfigurationHex)
	if err != nil {
		panic(err)
	}
	cfg, err := config.ParseBinaryConfig(defaultConfiguration)
	if err != nil {
		panic(err)
	}
	createConfiguration(cfg)
	finalConfiguration, err := config.WriteBinaryConfig(cfg)
	if err != nil {
		panic(err)
	}
	finalConfigurationHex := hex.EncodeToString(finalConfiguration)
	fmt.Println("Hex:")
	fmt.Println(finalConfigurationHex)
	fmt.Println("")
	fmt.Println("C code:")
	fmt.Println(`// Chip Configuration, generated with "make generate-atecc608-config"
// The first 16 bytes, as well as the LockValue/LockConfig can't be changed and are ignored when
// writing the configuration to the device. Locking is performed via the Lock command during setup,
// after writing the configuration.
// UserExtra and UserExtraAdd are setup automatically via the UpdateExtra command based on this
// configuration.
// The Counter0/Counter1 values are overwritten at setup via atcab_write_config_counter().
// Individual slot locking is performed at setup via atcab_lock_data_slot().
#if (ATCA_ECC_CONFIG_SIZE != 128)
#error "Unexpected configuration size"
#endif`)
	fmt.Println("// clang-format off")
	fmt.Println("static uint8_t _configuration[ATCA_ECC_CONFIG_SIZE] = {")
	for i := 0; i < 16; i++ {
		fmt.Print("    ")
		for j := 0; j < 8; j++ {
			fmt.Printf("0x%02x, ", finalConfiguration[i*8+j])
		}
		fmt.Print("\n")
	}
	fmt.Println("};")
	fmt.Println("// clang-format on")
}
