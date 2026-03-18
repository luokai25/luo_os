#include "timer.h"
#include "io.h"

static volatile uint32_t ticks = 0;

void timer_handler(void) {
    ticks++;
    outb(0x20, 0x20);
}

uint32_t timer_ticks(void) {
    return ticks;
}

void timer_init(void) {
    uint16_t divisor = 1193;
    outb(0x43, 0x36);
    outb(0x40, (uint8_t)(divisor & 0xFF));
    outb(0x40, (uint8_t)(divisor >> 8));
}
