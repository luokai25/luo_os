#include "serial.h"
#include "io.h"

#define COM1 0x3F8

void serial_init(void) {
    outb(COM1+1, 0x00);
    outb(COM1+3, 0x80);
    outb(COM1+0, 0x03);
    outb(COM1+1, 0x00);
    outb(COM1+3, 0x03);
    outb(COM1+2, 0xC7);
    outb(COM1+4, 0x0B);
}

uint8_t serial_received(void) {
    return inb(COM1+5) & 0x01;
}

char serial_getchar(void) {
    while (!serial_received());
    return (char)inb(COM1);
}

static uint8_t serial_tx_ready(void) {
    return inb(COM1+5) & 0x20;
}

void serial_putchar(char c) {
    while (!serial_tx_ready());
    if (c == '\n') {
        while (!serial_tx_ready());
        outb(COM1, '\r');
    }
    outb(COM1, (uint8_t)c);
}

void serial_print(const char* s) {
    for (int i = 0; s[i]; i++) serial_putchar(s[i]);
}

void serial_println(const char* s) {
    serial_print(s);
    serial_putchar('\n');
}

void serial_print_int(int n) {
    if (n < 0) { serial_putchar('-'); n = -n; }
    if (n == 0) { serial_putchar('0'); return; }
    char buf[12];
    int i = 0;
    while (n > 0) { buf[i++] = (char)('0' + n % 10); n /= 10; }
    while (i > 0) serial_putchar(buf[--i]);
}

void serial_print_hex(uint32_t n) {
    const char* h = "0123456789ABCDEF";
    serial_print("0x");
    for (int i = 28; i >= 0; i -= 4)
        serial_putchar(h[(n >> i) & 0xF]);
}

int serial_readline(char* buf, int max) {
    int len = 0;
    while (1) {
        char c = serial_getchar();
        if (c == '\r' || c == '\n') {
            buf[len] = '\0';
            serial_putchar('\n');
            return len;
        } else if ((c == 127 || c == 8) && len > 0) {
            len--;
            serial_print("\b \b");
        } else if (len < max - 1 && c >= 32) {
            buf[len++] = c;
            serial_putchar(c);
        }
    }
}
