#include "keyboard.h"
#include "io.h"

static const char sc_map[128] = {
    0,  27,'1','2','3','4','5','6','7','8','9','0','-','=',8,
    '\t','q','w','e','r','t','y','u','i','o','p','[',']','\n',
    0,'a','s','d','f','g','h','j','k','l',';','\'','`',
    0,'\\','z','x','c','v','b','n','m',',','.','/',0,
    '*',0,' '
};

static volatile char    last_key  = 0;
static volatile uint8_t key_ready = 0;

void keyboard_handler(void) {
    uint8_t sc = inb(0x60);
    if (!(sc & 0x80) && sc < 128 && sc_map[sc]) {
        last_key  = sc_map[sc];
        key_ready = 1;
    }
    outb(0x20, 0x20);
}

uint8_t keyboard_ready(void) { return key_ready; }

char keyboard_getchar(void) {
    while (!key_ready);
    key_ready = 0;
    return last_key;
}
