#!/bin/sh
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


# copied from libwally-core/tools and adapted to enable configuration from outside of the project.

autoreconf --install --force --warnings=all ${1}
if uname | grep "Darwin" >/dev/null 2>&1; then
    # Hack libtool to work around OSX requiring AR set to /usr/bin/libtool
    for f in ${1}./tools/build-aux/ltmain.sh ${1}./src/secp256k1/build-aux/ltmain.sh; do
        for a in x t; do
             sed -i -e "s/\$AR $a /ar $a /" $f
        done
    done
fi
