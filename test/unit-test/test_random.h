// SPDX-License-Identifier: Apache-2.0

#ifndef _TEST_RANDOM_H
#define _TEST_RANDOM_H

int __wrap_rand(void);
int __wrap_rust_sha256(const unsigned char* data, size_t len, unsigned char* out);

#endif
