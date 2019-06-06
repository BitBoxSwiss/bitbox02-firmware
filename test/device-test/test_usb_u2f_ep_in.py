#!/usr/bin/env python

from setup_test import *
import sys
import time

test_dir = os.path.dirname(os.path.realpath(__file__))
sys.path.insert(0, test_dir + '/../../py/')

from dbb_utils import *

def main(argv):
    test = test_dir + "/src/test_usb_u2f_ep_in.c"
    setup(argv, test)
    run_test(test)

def run_test(testfile):
    print("execute testcase for " + testfile)

    try:
        openSpecificHid(USB_HWW)
        time.sleep(5)
        reply = hid_send_and_read('Hi U2F!', 5)
        assert reply == 'Hi U2F!'

    except IOError as ex:
        print(ex)
    except(KeyboardInterrupt, SystemExit):
        print("Exiting code")
    dbb_hid.close()

if __name__ == "__main__":
    main(sys.argv[1:])
