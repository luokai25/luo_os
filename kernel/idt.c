#include "idt.h"
#include "io.h"

#define IDT_ENTRIES 256
static idt_entry_t idt[IDT_ENTRIES];
static idt_ptr_t   idt_ptr;

static void idt_set(int n, uint32_t base, uint16_t sel, uint8_t flags) {
    idt[n].base_low  = base & 0xFFFF;
    idt[n].base_high = (base >> 16) & 0xFFFF;
    idt[n].selector  = sel;
    idt[n].zero      = 0;
    idt[n].flags     = flags;
}

void idt_init(void) {
    idt_ptr.limit = sizeof(idt) - 1;
    idt_ptr.base  = (uint32_t)&idt;

    outb(0x20, 0x11); outb(0xA0, 0x11);
    outb(0x21, 0x20); outb(0xA1, 0x28);
    outb(0x21, 0x04); outb(0xA1, 0x02);
    outb(0x21, 0x01); outb(0xA1, 0x01);
    outb(0x21, 0xFC);
    outb(0xA1, 0xFF);

    extern void timer_isr(void);
    extern void keyboard_isr(void);
    idt_set(32, (uint32_t)timer_isr,    0x08, 0x8E);
    idt_set(33, (uint32_t)keyboard_isr, 0x08, 0x8E);

    __asm__ volatile ("lidt %0" :: "m"(idt_ptr));
    __asm__ volatile ("sti");
}
