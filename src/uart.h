#include <stdbool.h>
#include <stdint.h>
#include <utils_ringbuffer.h>

void uart_init(void);
int32_t uart_0_read(uint8_t* buf, uint16_t buf_len);
bool uart_0_write(const uint8_t* buf, uint16_t buf_len);
bool uart_0_write_from_queue(struct ringbuffer* queue);
