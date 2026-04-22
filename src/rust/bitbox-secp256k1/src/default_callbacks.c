// SPDX-License-Identifier: Apache-2.0

void secp256k1_default_illegal_callback_fn(const char* str, void* data)
{
    (void)str;
    (void)data;
    while (1) {
    }
}

void secp256k1_default_error_callback_fn(const char* str, void* data)
{
    (void)str;
    (void)data;
    while (1) {
    }
}
