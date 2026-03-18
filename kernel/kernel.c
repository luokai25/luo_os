#include <stdint.h>
#include "serial.h"
#include "idt.h"
#include "timer.h"
#include "memory.h"
#include "fs.h"
#include "process.h"
#include "shell.h"

void kernel_main(uint32_t magic, uint32_t multiboot_addr) {
    (void)magic;
    (void)multiboot_addr;

    serial_init();
    memory_init();
    fs_init();
    process_init();
    timer_init();
    idt_init();

    serial_println("luo_os v1.0 — booting...");
    serial_println("[ok] serial");
    serial_println("[ok] memory");
    serial_println("[ok] filesystem");
    serial_println("[ok] processes");
    serial_println("[ok] timer");
    serial_println("[ok] interrupts");
    serial_println("");

    shell_run();

    while (1) __asm__ volatile ("hlt");
}
