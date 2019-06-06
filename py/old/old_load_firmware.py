#!/usr/bin/env python3
# Copyright 2019 Shift Cryptosecurity AG
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.


import sys
import binascii
from dbb_utils import *

if len(sys.argv) is not 3:
    print('\n\nUsage:\n\tpython load_firmware.py firmware_name.bin firmware_version\n\n')
    sys.exit()
else:
    fn = sys.argv[1]
    version = sys.argv[2]


# Private key signatures (order is important)
if 'signed' in fn:
    print('\n\nPlease load the unsigned firmware binfile. Signatures are added within this script.\n\n')
    sys.exit()
elif version:
    sig = (
        '00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000'
        '00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000'
        '00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000'
        '00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000'
        '00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000'
        '00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000'
        '00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000'
        )
else:
    print('\n\nError: invalid firmware version ({}). Use the form \'vX.X.X\'\n\n'.format(version))
    sys.exit()


def printFirmwareHash(filename):
    with open(filename, "rb") as f:
        data = f.read()
        applen = applen_v2 if version == 2 else applen_v1
        data += b'\xFF' * (applen - len(data))
        print('\nHashed firmware', binascii.hexlify(Hash(data)))


# ----------------------------------------------------------------------------------
try:
    openHid()

    printFirmwareHash(fn)

    sendPlainBoot(b'b') # blink led
    sendPlainBoot(b'v') # bootloader version
    sendPlainBoot(b'e') # erase existing firmware (required)
    sendBin(fn)        # send new firmware

    # upload sigs and verify new firmware
    load_result = sendPlainBoot(b's\0' + binascii.unhexlify(sig))
    if load_result[1] == 'V':
        latest_version, = struct.unpack('>I', binascii.unhexlify(load_result[2+64:][:8]))
        app_version, = struct.unpack('>I', binascii.unhexlify(load_result[2+64+8:][:8]))
        print('ERROR: firmware downgrade not allowed. Got version %d, but must be equal or higher to %d' % (app_version, latest_version))
    elif load_result[1] != b'0'[0]:
        print('ERROR: invalid firmware signature\n\n')
    else:
        print('SUCCESS: valid firmware signature\n\n')
        sendPlainBoot(b'r') # reboot

except IOError as ex:
    print(ex)
except (KeyboardInterrupt, SystemExit):
    print('Exiting code')
