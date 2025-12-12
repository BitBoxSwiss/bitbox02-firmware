// SPDX-License-Identifier: Apache-2.0

#include "hww.h"
#include "memory/bitbox02_smarteeprom.h"
#include "usb/usb_packet.c"
#include "usb/usb_processing.c"
#include "usb/usb_processing.h"
#include "workflow/idle_workflow.h"
#include <fcntl.h>
#include <memory/memory.h>
#include <mock_memory.h>
#include <queue.h>
#include <random.h>
#include <rust/rust.h>
#include <sd.h>
#include <stdio.h>
#include <unistd.h>

#include <netdb.h>
#include <netinet/in.h>
#include <sys/socket.h>

#define BUFFER_SIZE 1024

int data_len;
int sockfd;

int get_usb_message_socket(uint8_t* input)
{
    return read(sockfd, input, USB_HID_REPORT_OUT_SIZE);
}

void send_usb_message_socket(void)
{
    const uint8_t* data = queue_pull(queue_hww_queue());
    while (data) {
        data_len = 256 * (int)data[5] + (int)data[6];
        if (!write(sockfd, data, USB_HID_REPORT_OUT_SIZE)) {
            perror("ERROR, could not write to socket");
            exit(1);
        }
        data = queue_pull(queue_hww_queue());
    }
}

void simulate_firmware_execution(const uint8_t* input)
{
    usb_packet_process((const USB_FRAME*)input);
    rust_workflow_spin();
    rust_async_usb_spin();
    usb_processing_process(usb_processing_hww());
}

int main(void)
{
    // Establish socket connection with client
    int portno = 15423;
    sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd < 0) {
        perror("ERROR opening socket");
        return 1;
    }
    struct hostent* server = gethostbyname("host.docker.internal");
    if (server == NULL) {
        fprintf(stderr, "ERROR, no such host\n");
        return 1;
    }
    struct sockaddr_in serv_addr;
    memset((char*)&serv_addr, 0, sizeof(serv_addr));
    serv_addr.sin_family = AF_INET;
    memcpy((char*)&serv_addr.sin_addr.s_addr, (char*)server->h_addr_list[0], server->h_length);
    serv_addr.sin_port = htons(portno);
    if (connect(sockfd, (struct sockaddr*)&serv_addr, sizeof(serv_addr)) < 0) {
        perror("ERROR, could not connect to client");
        return 1;
    }

    // BitBox02 simulation initializaition
    usb_processing_init();
    printf("USB setup success\n");

    hww_setup();
    printf("HWW setup success\n");

    bool sd_success = sd_format();
    printf("Sd card setup %s\n", sd_success ? "success" : "failed");
    if (!sd_success) {
        perror("ERROR, sd card setup failed");
        return 1;
    }

    mock_memory_factoryreset();
    memory_interface_functions_t ifs = {
        .random_32_bytes = random_32_bytes_mcu,
    };
    bool memory_success = memory_setup(&ifs);
    printf("Memory setup %s\n", memory_success ? "success" : "failed");
    if (!memory_success) {
        perror("ERROR, memory setup failed");
        return 1;
    }

    smarteeprom_bb02_config();
    bitbox02_smarteeprom_init();
    idle_workflow_blocking();

    // BitBox02 firmware loop
    uint8_t input[BUFFER_SIZE];
    int temp_len;
    while (1) {
        // Simulator polls for USB messages from client and then processes them
        if (!get_usb_message_socket(input)) break;
        simulate_firmware_execution(input);

        // If the USB message to be sent from firmware is bigger than one packet,
        // then the simulator sends the message in multiple packets. Packets use
        // HID format, just like the real USB messages.
        temp_len = data_len - (USB_HID_REPORT_OUT_SIZE - 7);
        while (temp_len > 0) {
            // When USB message processing function is called without a new
            // input, then it does not consume any packets but it still calls
            // the send function to send further USB messages
            usb_processing_process(usb_processing_hww());
            temp_len -= (USB_HID_REPORT_OUT_SIZE - 5);
        }
        send_usb_message_socket();
    }
    close(sockfd);
    return 0;
}
