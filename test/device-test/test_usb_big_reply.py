#!/usr/bin/env python

from setup_test import *
import sys
import time
import os

test_dir = os.path.dirname(os.path.realpath(__file__))
sys.path.insert(0, test_dir + '/../../py/')

from dbb_utils import *
from hid_u2f import *

def main(argv):
    test = test_dir + "/src/test_usb_cmd_process.c"
    setup(argv, test)
    run_test(test)

def run_test(testfile):
    print("execute testcase for " + testfile)

    try:
        openSpecificHid(USB_HWW)
        time.sleep(3)
        reply = hid_send_and_read('Hi HWW! 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0', 5)
        assert reply == 'Hi HWW! 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0'
        print(reply)
    except IOError as ex:
        print(ex)
    except(KeyboardInterrupt, SystemExit):
        print("Exiting code")
    dbb_hid.close()
#    time.sleep(3)
#
#    try:
#        openSpecificHid(USB_U2F)
#        time.sleep(3)
#        reply = hid_send_and_read_u2f_ping('ping u2f', 5)
#        print(reply)
#    except IOError as ex:
#        print(ex)
#    except(KeyboardInterrupt, SystemExit):
#        print("Exiting code")
#    dbb_hid.close()

if __name__ == "__main__":
    main(sys.argv[1:])
