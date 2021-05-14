// Copyright 2019 Shift Cryptosecurity AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include <memory/memory.h>

#define CHUNK_SIZE (8 * 1024)
#define NUM_CHUNKS (3)

static uint8_t _memory[CHUNK_SIZE * NUM_CHUNKS];

bool __wrap_memory_write_chunk_mock(uint32_t chunk_num, uint8_t* chunk)
{
    memcpy(&_memory[chunk_num * CHUNK_SIZE], chunk, CHUNK_SIZE);
    return true;
}

void __wrap_memory_read_chunk_mock(uint32_t chunk_num, uint8_t* chunk_out)
{
    memcpy(chunk_out, &_memory[chunk_num * CHUNK_SIZE], CHUNK_SIZE);
}

static void _reset_memory(void)
{
    memset(_memory, 0xFF, sizeof(_memory));
}

static void _test_memory_multisig(void** state)
{
    _reset_memory();

    const uint8_t hashes[][32] = {
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        "cccccccccccccccccccccccccccccccc",
        "dddddddddddddddddddddddddddddddd",
        "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
        "ffffffffffffffffffffffffffffffff",
    };
    const char* names[] = {
        "name1",
        "name2",
        "name3",
        "name4",
        "name5",
        "name6",
    };

    char name[31] = {0};
    assert_false(memory_multisig_get_by_hash(hashes[0], name));

    // set
    assert_int_equal(MEMORY_OK, memory_multisig_set_by_hash(hashes[0], names[0]));
    assert_int_equal(MEMORY_OK, memory_multisig_set_by_hash(hashes[1], names[1]));
    // overwrite with the same is possible
    assert_int_equal(MEMORY_OK, memory_multisig_set_by_hash(hashes[1], names[1]));

    // get
    assert_true(memory_multisig_get_by_hash(hashes[0], NULL));
    assert_true(memory_multisig_get_by_hash(hashes[0], name));
    assert_string_equal(name, names[0]);
    assert_true(memory_multisig_get_by_hash(hashes[1], name));
    assert_string_equal(name, names[1]);
    // rename
    const char* name0_renamed = "name 1 renamed";
    assert_int_equal(MEMORY_OK, memory_multisig_set_by_hash(hashes[0], name0_renamed));
    assert_true(memory_multisig_get_by_hash(hashes[0], name));
    assert_string_equal(name, name0_renamed);

    // rename to a name which already exists fails (duplicate name).
    assert_int_equal(MEMORY_ERR_DUPLICATE_NAME, memory_multisig_set_by_hash(hashes[0], names[1]));
    // was in fact not renamed
    assert_true(memory_multisig_get_by_hash(hashes[0], name));
    assert_string_equal(name, name0_renamed);
}

static void _test_memory_multisig_invalid(void** state)
{
    // invalid hash
    uint8_t empty[32];
    memset(empty, 0xFF, sizeof(empty));
    assert_int_equal(MEMORY_ERR_INVALID_INPUT, memory_multisig_set_by_hash(empty, "foo"));

    // invalid name
    uint8_t hash[32] = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    assert_int_equal(MEMORY_ERR_INVALID_INPUT, memory_multisig_set_by_hash(hash, ""));
}

static void _test_memory_multisig_full(void** state)
{
    _reset_memory();
    // Only 25 slots available.
    const size_t limit = 25;
    uint8_t hashes[limit + 1][32];
    char names[limit + 1][10];
    for (size_t i = 0; i < limit + 1; i++) {
        memset(hashes[i], i + i, 32);
        snprintf(names[i], sizeof(names[i]), "name%ld", i);
    }

    for (size_t i = 0; i < limit; i++) {
        assert_int_equal(MEMORY_OK, memory_multisig_set_by_hash(hashes[i], names[i]));
    }
    assert_int_equal(MEMORY_ERR_FULL, memory_multisig_set_by_hash(hashes[limit], names[limit]));
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_memory_multisig),
        cmocka_unit_test(_test_memory_multisig_invalid),
        cmocka_unit_test(_test_memory_multisig_full),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
