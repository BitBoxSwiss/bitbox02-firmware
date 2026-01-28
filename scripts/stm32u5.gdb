target extended-remote :3333

monitor reset halt

load

b HardFault_Handler

# Step into first instruction
si
