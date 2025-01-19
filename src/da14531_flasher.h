
#ifndef DA14531_FLASHER_H
#define DA14531_FLASHER_H

#include <stdbool.h>
#include <stdint.h>

enum flashing_state {
    FLASHING_STATE_IDLE,
    FLASHING_STATE_SEEN_STX,
    FLASHING_STATE_SENT_SOH,
    FLASHING_STATE_SEEN_SOH_ACK,
    FLASHING_STATE_SENT_FIRMWARE,
    FLASHING_STATE_SEND_CHKSUM_ACK,
    FLASHING_STATE_DISABLE_UART,
    FLASHING_STATE_ENABLE_UART,
    FLASHING_STATE_DONE_DONE,
};

struct Flasher {
    enum flashing_state state;
    uint8_t buf_in[16];
    uint16_t buf_in_len;
    uint8_t buf_out_small[16];
    const uint8_t* firmware_start;
    uint16_t firmware_size;
    uint8_t firmware_chksum;
    uint16_t bytes_sent;
};

// initialize flasher object
void flasher_init(struct Flasher* self, const uint8_t* firmware_start, uint16_t firmware_size);

const char* flashing_state_str(enum flashing_state state);

void flasher_poll(
    struct Flasher* self,
    const uint8_t* buf_in,
    uint16_t* buf_in_len,
    const uint8_t** buf_out,
    uint16_t* buf_out_len);

// This will issue a soft reset of the da14531 over SWD.
// This function only works if the debug interface is enabled.
void da14531_rst(void);
#endif
