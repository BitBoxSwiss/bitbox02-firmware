target extended-remote :3333

monitor reset init

load

b HardFault

# Step into first instruction
c
