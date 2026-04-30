# Helper function to start RTT server in openocd. Remember to let the firmware
# run until RTT channels have been initialized before running this command.
# Sets up channel 0 in/out for terminals and channel 1 for API requests/responses
define rtt_start
    monitor rtt setup 0x20000200 0x1000
    monitor rtt start
    monitor rtt server start 19021 0
# monitor rtt server start 19022 1
end
target extended-remote :3333

monitor reset init

load

# Step into first instruction
si
