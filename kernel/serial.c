#include "serial.h"
#include <stdint.h>

#define COM1 0x3F8

static inline void outb(uint16_t port, uint8_t val) {
    __asm__ volatile ("outb %0, %1" :: "a"(val), "Nd"(port));
}
static inline uint8_t inb(uint16_t port) {
    uint8_t val;
    __asm__ volatile ("inb %1, %0" : "=a"(val) : "Nd"(port));
    return val;
}

void serial_init(void) {
    outb(COM1 + 1, 0x00); /* disable interrupts */
    outb(COM1 + 3, 0x80); /* enable DLAB */
    outb(COM1 + 0, 0x03); /* baud 38400 lo */
    outb(COM1 + 1, 0x00); /* baud 38400 hi */
    outb(COM1 + 3, 0x03); /* 8 bits, no parity, 1 stop */
    outb(COM1 + 2, 0xC7); /* enable FIFO */
    outb(COM1 + 4, 0x0B); /* IRQs enabled, RTS/DSR set */
}

uint8_t serial_received(void) {
    return inb(COM1 + 5) & 0x01;
}

char serial_getchar(void) {
    while (!serial_received());
    return (char)inb(COM1);
}

static uint8_t serial_tx_ready(void) {
    return inb(COM1 + 5) & 0x20;
}

void serial_putchar(char c) {
    while (!serial_tx_ready());
    outb(COM1, (uint8_t)c);
}

void serial_print(const char* s) {
    for (int i = 0; s[i]; i++) {
        if (s[i] == '\n') serial_putchar('\r');
        serial_putchar(s[i]);
    }
}
