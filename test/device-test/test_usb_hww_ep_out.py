#!/usr/bin/env python

from setup_test import *
import sys
import time

test_dir = os.path.dirname(os.path.realpath(__file__))
sys.path.insert(0, test_dir + '/../../py/')

from dbb_utils import *

def main(argv):
    test = test_dir + "/src/test_usb_hww_ep_out.c"
    setup(argv, test)
    run_test(test)

def run_test(testfile):
    print("execute testcase for " + testfile)

    try:
        openSpecificHid(USB_HWW)
        hid_send_msg('{"if":"hww"}')

    except IOError as ex:
        print(ex)
    except(KeyboardInterrupt, SystemExit):
        print("Exiting code")
    dbb_hid.close()

if __name__ == "__main__":
    main(sys.argv[1:])
