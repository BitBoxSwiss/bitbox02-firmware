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


DIR=$(dirname $0)

SECP256K1_CFLAGS="-DUSE_BASIC_CONFIG=1 -DUSE_ECMULT_STATIC_PRECOMPUTATION=1"

${DIR}/libwally-autogen.sh ${DIR}/../libwally-core/ && \
    CFLAGS="-I${DIR}/../libwally-core/src/ccan/ -fdata-sections -ffunction-sections -Os -DNDEBUG -std=c11 -mthumb -fomit-frame-pointer -mlong-calls -mcpu=cortex-m4 -pipe -fno-strict-aliasing --param max-inline-insns-single=500 -DNDEBUG -fstack-protector-all -Wextra -Wall -Wpedantic -Wstrict-prototypes -Wmissing-prototypes -Werror-implicit-function-declaration -Wpointer-arith ${SECP256K1_CFLAGS}" \
    LDFLAGS="--specs=nosys.specs --specs=nano.specs -L${DIR}/../../src/drivers/lib/ssp -mthumb -lc -Wl,--gc-sections" \
    ${DIR}/../libwally-core/configure --host=arm-none-eabi --build=x86_64-linux-gnu --disable-tests --enable-static --disable-shared
