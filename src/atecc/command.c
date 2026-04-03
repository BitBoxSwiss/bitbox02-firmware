// SPDX-License-Identifier: Apache-2.0

#include "command.h"
#include <i2c_ecc.h>
#include <string.h>

static void _clear_ctx(atecc_command_ctx_t* ctx)
{
    ctx->device = NULL;
    ctx->packet = NULL;
    ctx->polls_remaining = 0;
    ctx->pending = false;
}

static void _idle_chip(void)
{
    uint8_t cmd = I2C_ECC_CHIP_IDLE;
    (void)i2c_ecc_write_one_shot(&cmd, 1);
}

ATCA_STATUS atecc_command_start(
    atecc_command_ctx_t* ctx,
    ATCAPacket* packet,
    ATCADevice device,
    uint16_t* wait_ms_out)
{
    uint8_t* txdata;

    if (ctx == NULL || packet == NULL || device == NULL || wait_ms_out == NULL) {
        return ATCA_BAD_PARAM;
    }

    memset(packet->data, 0, sizeof(packet->data));
    _clear_ctx(ctx);
    ctx->device = device;
    ctx->packet = packet;
    ctx->polls_remaining = ATCA_POLLING_MAX_TIME_MSEC / ATCA_POLLING_FREQUENCY_TIME_MSEC;
    ctx->pending = true;

    if (i2c_ecc_wake_one_shot() != I2C_ECC_WAKE) {
        _clear_ctx(ctx);
        return ATCA_COMM_FAIL;
    }

    txdata = (uint8_t*)packet;
    txdata[0] = I2C_ECC_CHIP_CMD;
    if (i2c_ecc_write_one_shot(txdata, (uint32_t)packet->txsize + 1) != 0) {
        _idle_chip();
        _clear_ctx(ctx);
        return ATCA_COMM_FAIL;
    }

    *wait_ms_out = ATCA_POLLING_INIT_TIME_MSEC;
    return ATCA_RX_NO_RESPONSE;
}

ATCA_STATUS atecc_command_poll(atecc_command_ctx_t* ctx, uint16_t* wait_ms_out)
{
    ATCA_STATUS status;
    uint16_t rxsize;

    if (ctx == NULL || wait_ms_out == NULL) {
        return ATCA_BAD_PARAM;
    }
    if (!ctx->pending || ctx->packet == NULL) {
        return ATCA_BAD_PARAM;
    }

    memset(ctx->packet->data, 0, sizeof(ctx->packet->data));
    rxsize = sizeof(ctx->packet->data);
    if (i2c_ecc_read_one_shot(ctx->packet->data, rxsize) != 0) {
        if (ctx->polls_remaining-- > 0) {
            *wait_ms_out = ATCA_POLLING_FREQUENCY_TIME_MSEC;
            return ATCA_RX_NO_RESPONSE;
        }
        _idle_chip();
        _clear_ctx(ctx);
        return ATCA_RX_NO_RESPONSE;
    }

    rxsize = ctx->packet->data[ATCA_COUNT_IDX];
    if (rxsize < 4 || rxsize > sizeof(ctx->packet->data)) {
        status = rxsize > 0 ? ATCA_RX_FAIL : ATCA_RX_NO_RESPONSE;
        _idle_chip();
        _clear_ctx(ctx);
        return status;
    }

    if ((status = atCheckCrc(ctx->packet->data)) != ATCA_SUCCESS) {
        _idle_chip();
        _clear_ctx(ctx);
        return status;
    }
    if ((status = isATCAError(ctx->packet->data)) != ATCA_SUCCESS) {
        _idle_chip();
        _clear_ctx(ctx);
        return status;
    }

    _idle_chip();
    _clear_ctx(ctx);
    return ATCA_SUCCESS;
}

void atecc_command_abort(atecc_command_ctx_t* ctx)
{
    if (ctx == NULL) {
        return;
    }
    if (ctx->pending) {
        _idle_chip();
    }
    _clear_ctx(ctx);
}

ATCA_STATUS atecc_command_execute(ATCAPacket* packet, ATCADevice device)
{
    atecc_command_ctx_t ctx = {0};
    ATCA_STATUS status;
    uint16_t wait_ms = 0;

    status = atecc_command_start(&ctx, packet, device, &wait_ms);
    while (status == ATCA_RX_NO_RESPONSE) {
        atca_delay_ms(wait_ms);
        status = atecc_command_poll(&ctx, &wait_ms);
    }
    return status;
}
