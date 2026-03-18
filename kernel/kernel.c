#include <stdint.h>

#define VGA_ADDRESS 0xB8000
#define VGA_WIDTH   80
#define VGA_HEIGHT  25
#define COLOR_WHITE_ON_BLACK 0x0F

static uint16_t* vga = (uint16_t*)VGA_ADDRESS;

static void vga_clear() {
    for (int i = 0; i < VGA_WIDTH * VGA_HEIGHT; i++)
        vga[i] = (COLOR_WHITE_ON_BLACK << 8) | ' ';
}

static void vga_print(const char* str, int row, int col) {
    for (int i = 0; str[i]; i++)
        vga[row * VGA_WIDTH + col + i] = (COLOR_WHITE_ON_BLACK << 8) | (uint8_t)str[i];
}

void kernel_main(uint32_t magic, uint32_t multiboot_addr) {
    (void)magic;
    (void)multiboot_addr;

    vga_clear();
    vga_print("=== LUO_OS KERNEL v0.1 ===",          1, 27);
    vga_print("Built from scratch. No limits.",       3, 24);
    vga_print("Human + AI Desktop Loading...",        5, 25);
    vga_print("[KERNEL]  Memory manager: ready",      8, 10);
    vga_print("[KERNEL]  VGA display:    ready",      9, 10);
    vga_print("[KERNEL]  Interrupts:     pending",   10, 10);
    vga_print("[KERNEL]  AI agent layer: pending",   11, 10);
    vga_print("Press any key to continue...",        23, 26);

    for (;;) __asm__("hlt");
}
