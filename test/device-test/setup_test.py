#!/usr/bin/env python

import sys
import getopt
import subprocess
import os
import shutil
import time

def print_help_and_exit(test_specified):
    print 'Options:'
    if test_specified is False:
        print '-t <test>'
        print '  Runs a specific test.'
    print '-r | --run-only'
    print '  Only runs the test case. Does not compile the test code or flash the device.'
    sys.exit(2)

def setup(argv, test):
    try:
        opts, args = getopt.getopt(argv,"ht:r",["test=", "run-only"])
    except getopt.GetoptError:
        print_help_and_exit(test is not None)

    run_only = False

    for opt, arg in opts:
       if opt == '-h':
           print_help_and_exit(test is not None)
       elif opt in ("-t", "--test"):
           test = arg
       elif opt in ("-r", "--run-only"):
           run_only = True

    if test is None or not test:
       print_help_and_exit(False)

    if run_only is False:
        exec_dir = os.getcwd();
        test_dir = setup_test_env()
        if not test.startswith("/"):
            test = exec_dir + "/" + test;
        build_test(test)
        flash_device(test_dir)
        time.sleep(5)

def setup_test_env():
    test_dir = os.path.dirname(os.path.realpath(__file__))
    test_build_dir = test_dir + "/build"

    if not os.path.isdir(test_build_dir):
        os.mkdir(test_build_dir)
    os.chdir(test_build_dir)
    return test_dir

def build_test(testfile):
    print 'main file: {}'.format(testfile);
    try:
        subprocess.check_call("cmake ../../../ -DBUILD_TYPE=device-test -DHARDWARE=v2 -DMAIN-SOURCE:STRING=" + testfile + " && make", shell=True)
    except subprocess.CalledProcessError as e:
        print("Compilation failed :-(")
        exit(e.returncode)

def flash_device(test_dir):
    try:
        #subprocess.check_call("JLinkExe -device ATSAMD51J20 -if SWD -speed 4000 -jtagconf -1,-1 -autoconnect 1 -CommanderScript " + test_dir + "/loadfirmware.jlink", shell=True)
        subprocess.check_call(test_dir + "/../../py/load_firmware.py " + test_dir + "/build/bin/device-test.bin debug", shell=True)
    except subprocess.CalledProcessError as e:
        print("Flashing the device failed :-(")
        exit(e.returncode)

if __name__ == "__main__":
    setup(sys.argv[1:], None)
