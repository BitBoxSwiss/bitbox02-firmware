// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include <string.h>

#include "u2f.c"

static void _set_confirming_state(uint8_t instruction, const USB_APDU* pending_apdu)
{
    memset(&_state, 0, sizeof(_state));
    _state.last_cmd = instruction;
    _set_pending_apdu(pending_apdu);
    if (instruction == U2F_REGISTER) {
        _state.reg = U2F_REGISTER_CONFIRMING;
    } else {
        _state.auth = U2F_AUTHENTICATE_CONFIRMING;
    }
}

static Packet _message(uint8_t instruction, uint32_t cid)
{
    Packet packet = {
        .len = sizeof(USB_APDU),
        .cmd = U2FHID_MSG,
        .cid = cid,
    };
    USB_APDU* apdu = (USB_APDU*)packet.data_addr;
    apdu->ins = instruction;
    return packet;
}

static Packet _register_message(uint32_t cid)
{
    Packet packet = {
        .len = sizeof(USB_APDU) + sizeof(U2F_REGISTER_REQ),
        .cmd = U2FHID_MSG,
        .cid = cid,
    };
    USB_APDU* apdu = (USB_APDU*)packet.data_addr;
    apdu->ins = U2F_REGISTER;
    apdu->p1 = U2F_AUTH_ENFORCE;
    apdu->lc3 = sizeof(U2F_REGISTER_REQ);
    U2F_REGISTER_REQ* request = (U2F_REGISTER_REQ*)apdu->data;
    memset(request->challenge, 0x11, sizeof(request->challenge));
    memset(request->appId, 0x22, sizeof(request->appId));
    return packet;
}

static Packet _authenticate_message(uint32_t cid)
{
    Packet packet = {
        .len = sizeof(USB_APDU) + U2F_AUTHENTICATE_REQ_LEN,
        .cmd = U2FHID_MSG,
        .cid = cid,
    };
    USB_APDU* apdu = (USB_APDU*)packet.data_addr;
    apdu->ins = U2F_AUTHENTICATE;
    apdu->p1 = U2F_AUTH_ENFORCE;
    apdu->lc3 = U2F_AUTHENTICATE_REQ_LEN;
    U2F_AUTHENTICATE_REQ* request = (U2F_AUTHENTICATE_REQ*)apdu->data;
    memset(request->challenge, 0x33, sizeof(request->challenge));
    memset(request->appId, 0x44, sizeof(request->appId));
    request->keyHandleLength = U2F_KEYHANDLE_LEN;
    memset(request->keyHandle, 0x55, request->keyHandleLength);
    return packet;
}

static void test_authenticate_bad_retry_preserves_confirmation(void** state)
{
    (void)state;

    Packet pending_packet = _authenticate_message(0x12345678);
    _set_confirming_state(U2F_AUTHENTICATE, (const USB_APDU*)pending_packet.data_addr);

    USB_APDU apdu = {
        .cla = 0,
        .ins = U2F_AUTHENTICATE,
    };
    Packet out_packet = {.cid = 0x12345678};
    _authenticate_continue(&apdu, &out_packet);

    assert_int_equal(_state.last_cmd, U2F_AUTHENTICATE);
    assert_int_equal(_state.auth, U2F_AUTHENTICATE_CONFIRMING);
    assert_true(_is_pending_apdu((const USB_APDU*)pending_packet.data_addr));
    assert_int_equal(out_packet.cid, 0x12345678);
    assert_int_equal(out_packet.len, 2);
    assert_int_equal(out_packet.data_addr[0], U2F_SW_WRONG_LENGTH >> 8);
    assert_int_equal(out_packet.data_addr[1], U2F_SW_WRONG_LENGTH & 0xff);
}

static void test_request_length_validation(void** state)
{
    (void)state;

    Packet register_packet = _register_message(0x12345678);
    USB_APDU* register_apdu = (USB_APDU*)register_packet.data_addr;
    register_apdu->lc3--;
    assert_int_equal(_register_sanity_check_req(register_apdu), U2F_SW_WRONG_LENGTH);
    register_apdu->lc3 += 2;
    assert_int_equal(_register_sanity_check_req(register_apdu), U2F_SW_WRONG_LENGTH);

    Packet authenticate_packet = _authenticate_message(0x12345678);
    USB_APDU* authenticate_apdu = (USB_APDU*)authenticate_packet.data_addr;
    U2F_AUTHENTICATE_REQ* authenticate_request = (U2F_AUTHENTICATE_REQ*)authenticate_apdu->data;

    authenticate_apdu->lc3 = offsetof(U2F_AUTHENTICATE_REQ, keyHandle) - 1;
    assert_int_equal(_authenticate_sanity_check_req(authenticate_apdu), U2F_SW_WRONG_LENGTH);
    authenticate_apdu->lc3 = U2F_AUTHENTICATE_REQ_LEN;
    authenticate_request->keyHandleLength--;
    assert_int_equal(_authenticate_sanity_check_req(authenticate_apdu), U2F_SW_WRONG_LENGTH);
}

static void test_same_request_can_continue_on_new_cid(void** state)
{
    (void)state;

    const uint32_t owner_cid = 0x12345678;
    const uint32_t other_cid = 0x87654321;
    Packet pending_packet = _register_message(owner_cid);
    const USB_APDU* pending_apdu = (const USB_APDU*)pending_packet.data_addr;
    _set_confirming_state(U2F_REGISTER, pending_apdu);

    Packet in_packet = pending_packet;
    in_packet.cid = other_cid;
    assert_true(_is_pending_apdu((const USB_APDU*)in_packet.data_addr));
    Packet out_packet = {0};
    _cmd_msg(&in_packet, &out_packet, sizeof(out_packet.data_addr));

    assert_int_equal(_state.last_cmd, U2F_REGISTER);
    assert_int_equal(_state.reg, U2F_REGISTER_CONFIRMING);
    assert_true(_is_pending_apdu(pending_apdu));
    assert_int_equal(out_packet.cid, other_cid);
    assert_int_equal(out_packet.len, 2);
    assert_int_equal(out_packet.data_addr[0], U2F_SW_CONDITIONS_NOT_SATISFIED >> 8);
    assert_int_equal(out_packet.data_addr[1], U2F_SW_CONDITIONS_NOT_SATISFIED & 0xff);
    usb_processing_unlock();
}

static void test_changed_request_cannot_continue_workflow(void** state)
{
    (void)state;

    Packet pending_packet = _authenticate_message(0x12345678);
    const USB_APDU* pending_apdu = (const USB_APDU*)pending_packet.data_addr;
    _set_confirming_state(U2F_AUTHENTICATE, pending_apdu);

    Packet changed_packet = pending_packet;
    changed_packet.cid = 0x87654321;
    USB_APDU* changed_apdu = (USB_APDU*)changed_packet.data_addr;
    U2F_AUTHENTICATE_REQ* changed_request = (U2F_AUTHENTICATE_REQ*)changed_apdu->data;

    changed_apdu->p1 ^= 1;
    assert_false(_is_pending_apdu(changed_apdu));
    changed_apdu->p1 ^= 1;
    changed_request->challenge[0] ^= 1;
    assert_false(_is_pending_apdu(changed_apdu));
    changed_request->challenge[0] ^= 1;
    changed_request->appId[0] ^= 1;
    assert_false(_is_pending_apdu(changed_apdu));
    changed_request->appId[0] ^= 1;
    changed_request->keyHandle[0] ^= 1;
    assert_false(_is_pending_apdu(changed_apdu));
    changed_request->keyHandle[0] ^= 1;
    changed_apdu->lc3--;
    assert_false(_is_pending_apdu(changed_apdu));
    changed_apdu->lc3++;
    assert_true(_is_pending_apdu(changed_apdu));

    changed_request->challenge[0] ^= 1;
    Packet out_packet = {0};
    _cmd_msg(&changed_packet, &out_packet, sizeof(out_packet.data_addr));

    assert_int_equal(_state.last_cmd, U2F_AUTHENTICATE);
    assert_int_equal(_state.auth, U2F_AUTHENTICATE_CONFIRMING);
    assert_true(_is_pending_apdu(pending_apdu));
    assert_int_equal(out_packet.cid, changed_packet.cid);
    assert_int_equal(out_packet.len, 2);
    assert_int_equal(out_packet.data_addr[0], U2F_SW_CONDITIONS_NOT_SATISFIED >> 8);
    assert_int_equal(out_packet.data_addr[1], U2F_SW_CONDITIONS_NOT_SATISFIED & 0xff);
    usb_processing_unlock();
}

static void test_cross_command_cannot_continue_workflow(void** state)
{
    (void)state;

    Packet pending_packet = _authenticate_message(0x12345678);
    const USB_APDU* pending_apdu = (const USB_APDU*)pending_packet.data_addr;
    _set_confirming_state(U2F_AUTHENTICATE, pending_apdu);

    Packet in_packet = _register_message(0x87654321);
    Packet out_packet = {0};
    _cmd_msg(&in_packet, &out_packet, sizeof(out_packet.data_addr));

    assert_int_equal(_state.last_cmd, U2F_AUTHENTICATE);
    assert_int_equal(_state.auth, U2F_AUTHENTICATE_CONFIRMING);
    assert_true(_is_pending_apdu(pending_apdu));
    assert_int_equal(out_packet.cid, in_packet.cid);
    assert_int_equal(out_packet.data_addr[0], U2F_SW_CONDITIONS_NOT_SATISFIED >> 8);
    assert_int_equal(out_packet.data_addr[1], U2F_SW_CONDITIONS_NOT_SATISFIED & 0xff);
    usb_processing_unlock();
}

static void test_stateless_request_preserves_workflow(void** state)
{
    (void)state;

    const uint32_t other_cid = 0x87654321;
    Packet pending_packet = _authenticate_message(0x12345678);
    const USB_APDU* pending_apdu = (const USB_APDU*)pending_packet.data_addr;
    _set_confirming_state(U2F_AUTHENTICATE, pending_apdu);

    Packet in_packet = _message(U2F_VERSION, other_cid);
    Packet out_packet = {0};
    _cmd_msg(&in_packet, &out_packet, sizeof(out_packet.data_addr));

    assert_int_equal(_state.last_cmd, U2F_AUTHENTICATE);
    assert_int_equal(_state.auth, U2F_AUTHENTICATE_CONFIRMING);
    assert_true(_is_pending_apdu(pending_apdu));
    assert_int_equal(out_packet.cid, other_cid);
    usb_processing_unlock();
}

static void test_malformed_message_preserves_workflow(void** state)
{
    (void)state;

    Packet pending_packet = _authenticate_message(0x12345678);
    const USB_APDU* pending_apdu = (const USB_APDU*)pending_packet.data_addr;
    _set_confirming_state(U2F_AUTHENTICATE, pending_apdu);

    Packet in_packet = _message(U2F_VERSION, 0x87654321);
    ((USB_APDU*)in_packet.data_addr)->lc3 = 1;
    Packet out_packet = {0};
    _cmd_msg(&in_packet, &out_packet, sizeof(out_packet.data_addr));

    assert_int_equal(_state.last_cmd, U2F_AUTHENTICATE);
    assert_int_equal(_state.auth, U2F_AUTHENTICATE_CONFIRMING);
    assert_true(_is_pending_apdu(pending_apdu));
    assert_int_equal(out_packet.len, 0);
}

static void test_init_preserves_workflow(void** state)
{
    (void)state;

    Packet pending_packet = _authenticate_message(0x12345678);
    const USB_APDU* pending_apdu = (const USB_APDU*)pending_packet.data_addr;
    _set_confirming_state(U2F_AUTHENTICATE, pending_apdu);

    Packet in_packet = {
        .len = sizeof(U2FHID_INIT_REQ),
        .cmd = U2FHID_INIT,
        .cid = U2FHID_CID_BROADCAST,
    };
    Packet out_packet = {0};
    _cmd_init(&in_packet, &out_packet, sizeof(out_packet.data_addr));

    const U2FHID_INIT_RESP* response = (const U2FHID_INIT_RESP*)out_packet.data_addr;
    assert_int_not_equal(response->cid, 0);
    assert_int_not_equal(response->cid, U2FHID_CID_BROADCAST);
    assert_int_equal(_state.last_cmd, U2F_AUTHENTICATE);
    assert_int_equal(_state.auth, U2F_AUTHENTICATE_CONFIRMING);
    assert_true(_is_pending_apdu(pending_apdu));
    assert_int_equal(out_packet.cid, U2FHID_CID_BROADCAST);
    usb_processing_unlock();
}

static void test_blocked_error_uses_request_cid(void** state)
{
    (void)state;

    const uint32_t owner_cid = 0x12345678;
    const uint32_t request_cid = 0x87654321;
    Packet pending_packet = _authenticate_message(owner_cid);
    const USB_APDU* pending_apdu = (const USB_APDU*)pending_packet.data_addr;
    _set_confirming_state(U2F_AUTHENTICATE, pending_apdu);

    Packet in_packet = _message(U2F_AUTHENTICATE, request_cid);
    Packet out_packet = {0};
    u2f_blocked_req_error(&out_packet, &in_packet);

    assert_int_equal(_state.last_cmd, U2F_AUTHENTICATE);
    assert_int_equal(_state.auth, U2F_AUTHENTICATE_CONFIRMING);
    assert_true(_is_pending_apdu(pending_apdu));
    assert_int_equal(out_packet.cid, request_cid);
    assert_int_equal(out_packet.len, 2);
    assert_int_equal(out_packet.data_addr[0], U2F_SW_CONDITIONS_NOT_SATISFIED >> 8);
    assert_int_equal(out_packet.data_addr[1], U2F_SW_CONDITIONS_NOT_SATISFIED & 0xff);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_authenticate_bad_retry_preserves_confirmation),
        cmocka_unit_test(test_request_length_validation),
        cmocka_unit_test(test_same_request_can_continue_on_new_cid),
        cmocka_unit_test(test_changed_request_cannot_continue_workflow),
        cmocka_unit_test(test_cross_command_cannot_continue_workflow),
        cmocka_unit_test(test_stateless_request_preserves_workflow),
        cmocka_unit_test(test_malformed_message_preserves_workflow),
        cmocka_unit_test(test_init_preserves_workflow),
        cmocka_unit_test(test_blocked_error_uses_request_cid),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
