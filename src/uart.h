#include <stdbool.h>
#include <stdint.h>

void uart_init(void);
int32_t uart_0_read(uint8_t* buf, uint16_t buf_len);
bool uart_0_write(const uint8_t* buf, uint16_t buf_len);
