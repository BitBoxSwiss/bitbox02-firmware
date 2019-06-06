#!/usr/bin/env python
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


import os
import sys
import json
import base64
import pyaes
import hid # hidapi (requires cython)
import hashlib
import struct

import time
import binascii

# ----------------------------------------------------------------------------------
#

# TODO: update accordingly
v2_serial_number = "dbb.fw:v0.0.1"
applen_v2 = 0xF0000       # BitBox_v2 firmware size
applen_v1 = 0x37000       # BitBox_v1 firmware size
chunksize = 8*512
usb_report_size = 64 # firmware > v2.0
# v1 size for boot commands:
boot_buf_size_send = 4098
boot_buf_size_reply = 256

class Usage:
    """The USB usage"""
    interface = 1
    usage_page = 0xFFFF

    def __init__(self, interface, usage_page):
        self.interface = interface
        self.usage_page = usage_page

USB_HWW = Usage(0, 0xFFFF)
USB_U2F = Usage(1, 0xD0F1)

# ----------------------------------------------------------------------------------
# Crypto
#

def aes_encrypt_with_iv(key, iv, data):
    aes_cbc = pyaes.AESModeOfOperationCBC(key, iv=iv)
    aes = pyaes.Encrypter(aes_cbc)
    e = aes.feed(data) + aes.feed()  # empty aes.feed() appends pkcs padding
    return e


def aes_decrypt_with_iv(key, iv, data):
    aes_cbc = pyaes.AESModeOfOperationCBC(key, iv=iv)
    aes = pyaes.Decrypter(aes_cbc)
    s = aes.feed(data) + aes.feed()  # empty aes.feed() strips pkcs padding
    return s


def EncodeAES(secret, s):
    iv = bytes(os.urandom(16))
    ct = aes_encrypt_with_iv(secret, iv, s)
    e = iv + ct
    return base64.b64encode(e)


def DecodeAES(secret, e):
    e = bytes(base64.b64decode(e))
    iv, e = e[:16], e[16:]
    s = aes_decrypt_with_iv(secret, iv, e)
    return s


def sha256(x):
    return hashlib.sha256(x).digest()


def Hash(x):
    if type(x) is bytes: return sha256(sha256(x))
    if type(x) is not bytearray: x=x.encode('utf-8')
    return sha256(sha256(x))


# ----------------------------------------------------------------------------------
# HID
#
def getHidPath(usage):
    for d in hid.enumerate(0, 0):
        if d['vendor_id'] == 0x03eb and d['product_id'] in (0x2402, 0x2403):
            if d['interface_number'] == usage.interface or d['usage_page'] == usage.usage_page:
                # hidapi is not consistent across platforms
                # usage_page works on Windows/Mac; interface_number works on Linux
                return d['path']


dbb_hid = hid.device()
dbb_version = None;

def identifyVersion():
    global dbb_version
    serial_number = dbb_hid.get_serial_number_string()
    if serial_number == v2_serial_number:
        dbb_version = 2
    else:
        dbb_version = 1

def openHid():
    openSpecificHid(USB_HWW)

def closeHid():
    dbb_hid.close();

def openSpecificHid(usage):
    print("\nOpening device")
    try:
        dbb_hid.open_path(getHidPath(usage))
        print("\tManufacturer: %s" % dbb_hid.get_manufacturer_string())
        print("\tProduct: %s" % dbb_hid.get_product_string())
        print("\tSerial No: %s\n\n" % dbb_hid.get_serial_number_string())
        identifyVersion()
        print("\tBitBox Version No: %s\n\n" % dbb_version)
    except Exception as e:
        print("\nDevice not found: (%s)\n" % str(e))
        sys.exit()


# ----------------------------------------------------------------------------------
# ISO 7816-4
#

HWW_CID = 0xFF000000
HWW_CMD = 0x80 + 0x40 + 0x01

U2F_PING_CMD = 0x80 + 0x01

def hid_send_frames(cmd, data):
    data = bytearray(data)
    data_len = len(data)
    seq = 0;
    idx = 0;
    write = []
    while idx < data_len:
        if idx == 0:
            # INIT frame
            write = data[idx : idx + min(data_len, usb_report_size - 7)]
            dbb_hid.write(b'\0' + struct.pack(">IBH",HWW_CID, cmd, data_len & 0xFFFF) + write + b'\xEE' * (usb_report_size - 7 - len(write)))
        else:
            # CONT frame
            write = data[idx : idx + min(data_len, usb_report_size - 5)]
            dbb_hid.write(b'\0' + struct.pack(">IB", HWW_CID, seq) + write + b'\xEE' * (usb_report_size - 5 - len(write)))
            seq += 1
        idx += len(write)


def hid_read_frames(cmd=HWW_CMD, timeout=5):
    # INIT response
    if timeout is None:
        timeout = 30;
    timeout_ms = timeout * 1000
    read = dbb_hid.read(usb_report_size, timeout_ms)
    if len(read) >= 3:
        cid = ((read[0] * 256 + read[1]) * 256 + read[2]) * 256 + read[3]
        reply_cmd = read[4]
        data_len = read[5] * 256 + read[6]
        data = read[7:]
        idx = len(read) - 7;
        while idx < data_len:
            # CONT response
            read = dbb_hid.read(usb_report_size, timeout_ms)
            if len(read) < 3:
                raise Exception('Did not receive a continuation frame after %d seconds' % timeout)
            data += read[5:]
            idx += len(read) - 5
        assert cid == HWW_CID, '- USB command ID mismatch'
        assert reply_cmd == cmd, '- USB command frame mismatch'
        return data
    else:
        raise Exception('Did not read anything after %d seconds' % timeout)

# ----------------------------------------------------------------------------------
# Firmware API (keep consistent with the Electrum plugin)
#

def hid_read(cmd=HWW_CMD, timeout=None):
    try:
        reply = hid_read_frames(cmd, timeout)
        reply = bytearray(reply).rstrip(b' \t\r\n\0')
    except Exception as e:
        reply = ''
        print('Exception caught: ' + str(e))
    return reply

def hid_read_json(timeout=None):
    try:
        r = hid_read(HWW_CMD, timeout)
        r = ''.join(chr(e) for e in r)
        reply = json.loads(r)
        print("JSON:   {}".format(reply))
    except Exception as e:
        reply = ''
        print('Exception caught: ' + str(e))
    return reply


def hid_send_msg(msg, cmd=HWW_CMD):
    if type(msg) == str:
        msg = msg.encode()
    try:
        serial_number = dbb_hid.get_serial_number_string()
        if serial_number == "dbb.fw:v2.0.0" or serial_number == "dbb.fw:v1.3.2" or serial_number == "dbb.fw:v1.3.1":
            print('Please upgrade your firmware: digitalbitbox.com/firmware')
            sys.exit()
        hid_send_frames(cmd, msg)
    except Exception as e:
        print('Exception caught: ' + str(e))

def hid_send_and_read(msg, timeout=None):
    hid_send_msg(msg)
    return hid_read(HWW_CMD, timeout)

def hid_send_and_read_json(msg, timeout=None):
    hid_send_msg(msg)
    return hid_read_json(timeout)

def hid_send_encrypt(msg, password, timeout=None):
    print("Sending: {}".format(msg))
    reply = ""
    try:
        secret = Hash(password)
        msg = EncodeAES(secret, msg)
        reply = hid_send_and_read_json(msg, timeout)
        if 'ciphertext' in reply:
            reply = DecodeAES(secret, ''.join(reply["ciphertext"]))
            print("Reply:   {}\n".format(reply))
            reply = json.loads(reply)
        if 'error' in reply:
            password = None
            print("\n\nReply:   {}\n\n".format(reply))
    except Exception as e:
        print('Exception caught ' + str(e))
    return reply


# ----------------------------------------------------------------------------------
# Bootloader API
#

def sendBoot(msg, timeout=None):
    if dbb_version == 2:
        hid_send_frames(HWW_CMD, bytearray(msg))
        reply = bytes(hid_read_frames(HWW_CMD, timeout))
        #reply = ''.join(chr(e) for e in reply)
    elif dbb_version == 1:
        msg = bytearray(msg) + b'\0' * (boot_buf_size_send - len(msg))
        dbb_hid.write(b'\0' + msg)
        reply = []
        while len(reply) < boot_buf_size_reply:
            reply = reply + dbb_hid.read(boot_buf_size_reply)
        reply = bytearray(reply).rstrip(b' \t\r\n\0')
        reply = ''.join(chr(e) for e in reply)
    else:
        print("\nBootloader version error\n\n")
        sys.exit()
    return reply

def sendPlainBoot(msg):
    try:
        print("\nSending: {}".format(msg[:2]))
        reply = sendBoot(msg)
        if msg.startswith(b's'):
            print("Reply:   {} {} (firmware hash)\n".format(reply[:2], binascii.hexlify(reply[2:34])))
        else:
            print("Reply:   {} {}\n".format(reply[:2], reply[2:]))
        return reply
    except Exception as e:
        print('Exception caught ' + str(e))
        return "";


def sendChunk(chunknum, data):
    try:
        # \x77 = 'w'
        b = bytearray(b"\x77\x00")
        b[1] = chunknum % 0xFF
        b.extend(data)
        reply = sendBoot(b)
        print("Loaded: {}  Code: {}".format(chunknum, reply))
    except Exception as e:
        print('Exception caught ' + str(e))


def sendBin(filename):
    with open(filename, "rb") as f:
        cnt = 0
        while True:
            data = f.read(chunksize)
            if len(data) == 0:
                break
            sendChunk(cnt, data)
            cnt += 1
