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
    test = test_dir + "/src/test_confirm_bip39.c"
    setup(argv, test)
    run_test(test)

def run_test(testfile):
    print("execute testcase for " + testfile)

if __name__ == "__main__":
    main(sys.argv[1:])
