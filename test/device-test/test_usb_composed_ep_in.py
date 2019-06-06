#!/usr/bin/env python

from setup_test import *
import sys
import time
import os

test_dir = os.path.dirname(os.path.realpath(__file__))
sys.path.insert(0, test_dir + '/../../py/')

from dbb_utils import *

def main(argv):
    test = test_dir + "/src/test_usb_composed_ep_in.c"
    setup(argv, test)
    run_test(test)

def run_test(testfile):
    print("execute testcase for " + testfile)

    interfaces = [USB_HWW, USB_U2F]

    for interface in interfaces:
        try:
            openSpecificHid(interface)

            # Example JSON command
            if interface is USB_HWW:
                message = 'Hi HWW!'
            elif interface is USB_U2F:
                message = 'Hi U2F!'

            reply = hid_send_and_read(message, 5)
            assert reply == message

        except IOError as ex:
            print(ex)
        except(KeyboardInterrupt, SystemExit):
            print("Exiting code")
        dbb_hid.close()
        time.sleep(2)

if __name__ == "__main__":
    main(sys.argv[1:])
