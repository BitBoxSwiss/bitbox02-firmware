#include "uart.h"
#include "driver_init.h"
#include "util.h"
#include "utils_assert.h"

#define EVENT_READ 0x01 // Available to read
#define EVENT_WRITE 0x02 // Available to write

volatile int usart_0_readyness = 0;
volatile uint32_t usart_0_status = 0;

struct io_descriptor* io;

static void rx_cb(const struct usart_async_descriptor* const descr)
{
    if (descr == &USART_0) {
        usart_0_readyness |= EVENT_READ;
    }
}

static void tx_cb(const struct usart_async_descriptor* const descr)
{
    if (descr == &USART_0) {
        usart_0_readyness |= EVENT_WRITE;
    }
}

// #pragma GCC diagnostic push
// #pragma GCC diagnostic ignored "-Wdiscarded-qualifiers"
// static void err_cb(const struct usart_async_descriptor* const descr)
//{
//     if (descr == &USART_0) {
//         uint32_t status_reg = _usart_async_get_status(&descr->device);
//         // uint32_t status_reg = hri_sercomusart_read_RXERRCNT_reg(&descr->device);
//         util_log("usart status: %08x", (unsigned int)status_reg);
//         // usart_async_disable(descr);
//         //  struct usart_async_status stat;
//         //  usart_async_get_status(descr, &stat);
//         //  util_log(
//         //      "uart error flags: %08x, rx:%08x, tx:%08x",
//         //      (unsigned int)stat.flags,
//         //      (unsigned int)stat.rxcnt,
//         //      (unsigned int)stat.txcnt);
//         //  usart_0_status = descr->stat;
//         //   if (stat.rxcnt == USART_0_BUFFER_SIZE) {
//         //       usart_async_flush_rx_buffer(descr);
//         //   }
//     }
// }
// #pragma GCC diagnostic pop

// static uint8_t filter_ascii(uint8_t c)
//{
//     return c >= 0x20 && c < 0x7f ? c : ' ';
// }

static int32_t _read(uint8_t* buf, uint16_t buf_len)
{
    int32_t read = 0;
    CRITICAL_SECTION_ENTER()
    usart_0_readyness &= ~EVENT_READ;
    read = io_read(io, buf, buf_len);
    CRITICAL_SECTION_LEAVE()

    // There was supposed to be data...
    if (read == 0) {
        util_log("Got spurious interrupt");
        return 0;
    }

    // util_log("[RX] (%db): ", (int)read);
    // for (int i = 0; i < read; i++) {
    //     util_log("%02X", buf[i]);
    // }
    // util_log("\n");
    // util_log("[RX] (%db): ", read);
    // for (int i = 0; i < read; i++) {
    //    util_log("%c", filter_ascii(buf[i]));
    // }
    // util_log("");
    return read;
}

static int32_t _write(const uint8_t* buf, uint16_t buf_len)
{
    ASSERT(buf && buf_len);
    int16_t wrote = 0;
    CRITICAL_SECTION_ENTER()
    wrote = io_write(io, buf, buf_len);
    usart_0_readyness &= ~EVENT_WRITE;
    CRITICAL_SECTION_LEAVE()

    ASSERT(wrote == buf_len); // TODO: handle partial writes

    // util_log("[TX] (%db): ", wrote);
    // for (int i = 0; i < MIN(64, wrote); i++) {
    //     util_log("%02X", buf[i]);
    // }
    // util_log("\n");
    return wrote;
}

void uart_init(void)
{
    util_log("uart_init");
    usart_async_get_io_descriptor(&USART_0, &io);
    usart_async_register_callback(&USART_0, USART_ASYNC_RXC_CB, rx_cb);
    usart_async_register_callback(&USART_0, USART_ASYNC_TXC_CB, tx_cb);
    // usart_async_register_callback(&USART_0, USART_ASYNC_ERROR_CB, err_cb);
    // usart_async_enable(&USART_0);

    usart_0_readyness |= EVENT_WRITE;
}

// Asynchronous read function
int32_t uart_0_read(uint8_t* buf, uint16_t buf_len)
{
    if (usart_0_readyness & EVENT_READ) {
        return _read(buf, buf_len);
    }
    return 0;
}

bool uart_0_write(const uint8_t* buf, uint16_t buf_len)
{
    if (!(usart_0_readyness & EVENT_WRITE)) {
        return false;
    }
    int32_t wrote = _write(buf, buf_len);
    ASSERT(wrote == buf_len);
    return wrote == buf_len;
}

static uint8_t out_buf[256];

bool uart_0_write_from_queue(struct ringbuffer* queue)
{
    if (!(usart_0_readyness & EVENT_WRITE)) {
        return false;
    }
    int32_t len;
    CRITICAL_SECTION_ENTER()
    len = MIN(ringbuffer_num(queue), sizeof(out_buf));
    CRITICAL_SECTION_LEAVE()
    for (int32_t i = 0; i < len; i++) {
        int32_t res = ringbuffer_get(queue, &out_buf[i]);
        ASSERT(res == ERR_NONE);
        if (res != ERR_NONE) {
            break;
        }
    }
    int32_t wrote = _write(out_buf, len);
    ASSERT(wrote == len);
    return wrote == len;
}
