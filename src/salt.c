#include "salt.h"
#include "memory/memory.h"
#include "util.h"

#include <string.h>

#include <crypto/sha2/sha256.h>

bool salt_hash_data(const uint8_t* data, size_t data_len, const char* purpose, uint8_t* hash_out)
{
    if (data_len > 0 && data == NULL) {
        return false;
    }
    if (!purpose || !hash_out) {
        return false;
    }

    uint8_t salt_root[32];
    UTIL_CLEANUP_32(salt_root);
    if (!memory_get_salt_root(salt_root)) {
        return false;
    }

    sha256_context_t ctx;
    sha256_reset(&ctx);
    noise_sha256_update(&ctx, salt_root, sizeof(salt_root));
    noise_sha256_update(&ctx, purpose, strlen(purpose));
    if (data != NULL) {
        noise_sha256_update(&ctx, data, data_len);
    }
    sha256_finish(&ctx, hash_out);
    return true;
}
