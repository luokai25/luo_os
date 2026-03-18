#include "keyboard.h"
#include <stdint.h>

static inline uint8_t inb(uint16_t port) {
    uint8_t val;
    __asm__ volatile ("inb %1, %0" : "=a"(val) : "Nd"(port));
    return val;
}

static inline void outb(uint16_t port, uint8_t val) {
    __asm__ volatile ("outb %0, %1" :: "a"(val), "Nd"(port));
}

static const char sc_map[128] = {
    0,  27, '1','2','3','4','5','6','7','8','9','0','-','=', 8,
    '\t','q','w','e','r','t','y','u','i','o','p','[',']','\n',
    0,  'a','s','d','f','g','h','j','k','l',';','\'','`',
    0, '\\','z','x','c','v','b','n','m',',','.','/', 0,
    '*', 0, ' '
};

static char last_key = 0;

void keyboard_handler(void) {
    uint8_t sc = inb(0x60);
    if (!(sc & 0x80) && sc < 128)
        last_key = sc_map[sc];
    outb(0x20, 0x20);
}

char keyboard_getchar(void) {
    char c = last_key;
    last_key = 0;
    return c;
}
