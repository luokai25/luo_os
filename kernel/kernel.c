#include <stdint.h>
#include "serial.h"

#define VGA_ADDRESS 0xB8000
#define VGA_WIDTH   80
#define VGA_HEIGHT  25
#define COLOR_WHITE 0x0F
#define COLOR_GREEN 0x0A
#define COLOR_CYAN  0x0B

static uint16_t* vga = (uint16_t*)VGA_ADDRESS;

static void vga_clear() {
    for (int i = 0; i < VGA_WIDTH * VGA_HEIGHT; i++)
        vga[i] = (COLOR_WHITE << 8) | ' ';
}

static void vga_putchar(char c, int row, int col, uint8_t color) {
    vga[row * VGA_WIDTH + col] = (color << 8) | (uint8_t)c;
}

static void vga_print(const char* s, int row, int col, uint8_t color) {
    for (int i = 0; s[i]; i++)
        vga_putchar(s[i], row, col + i, color);
}

static void shell_run(void) {
    char buf[80];
    int  len = 0;

    serial_print("\r\n");
    serial_print("  ██╗     ██╗   ██╗ ██████╗      ██████╗ ███████╗\r\n");
    serial_print("  ██║     ██║   ██║██╔═══██╗    ██╔═══██╗██╔════╝\r\n");
    serial_print("  ██║     ██║   ██║██║   ██║    ██║   ██║███████╗\r\n");
    serial_print("  ██║     ██║   ██║██║   ██║    ██║   ██║╚════██║\r\n");
    serial_print("  ███████╗╚██████╔╝╚██████╔╝    ╚██████╔╝███████║\r\n");
    serial_print("  ╚══════╝ ╚═════╝  ╚═════╝      ╚═════╝ ╚══════╝\r\n");
    serial_print("\r\n");
    serial_print("  LUO_OS v0.3 — Serial Shell\r\n");
    serial_print("  Type 'help' for commands\r\n");
    serial_print("\r\n");

    while (1) {
        serial_print("luo_os> ");
        len = 0;

        while (1) {
            char c = serial_getchar();
            if (c == '\r' || c == '\n') {
                serial_print("\r\n");
                buf[len] = '\0';
                break;
            } else if (c == 127 || c == 8) {
                if (len > 0) {
                    len--;
                    serial_print("\b \b");
                }
            } else if (len < 79) {
                buf[len++] = c;
                serial_putchar(c);
            }
        }

        /* commands */
        if (len == 0) continue;

        if (buf[0]=='h' && buf[1]=='e' && buf[2]=='l' && buf[3]=='p' && buf[4]=='\0') {
            serial_print("  help     — show this list\r\n");
            serial_print("  version  — OS version\r\n");
            serial_print("  clear    — clear screen\r\n");
            serial_print("  about    — about luo_os\r\n");
        } else if (buf[0]=='v' && buf[1]=='e' && buf[2]=='r') {
            serial_print("  luo_os v0.3 — kernel + serial shell\r\n");
        } else if (buf[0]=='a' && buf[1]=='b' && buf[2]=='o' && buf[3]=='u' && buf[4]=='t') {
            serial_print("  Built from scratch by luokai25\r\n");
            serial_print("  Stack: ASM + C + Rust + Python\r\n");
            serial_print("  Goal:  Human + AI desktop OS\r\n");
        } else if (buf[0]=='c' && buf[1]=='l' && buf[2]=='e' && buf[3]=='a' && buf[4]=='r') {
            serial_print("\033[2J\033[H");
        } else {
            serial_print("  unknown command: ");
            serial_print(buf);
            serial_print("\r\n  type 'help'\r\n");
        }
    }
}

void kernel_main(uint32_t magic, uint32_t multiboot_addr) {
    (void)magic; (void)multiboot_addr;

    serial_init();

    vga_clear();
    vga_print("=== LUO_OS v0.3 ===",           1, 30, COLOR_CYAN);
    vga_print("Serial shell active on COM1",    3, 26, COLOR_WHITE);
    vga_print("[KERNEL]  Serial:      ready",   5, 10, COLOR_WHITE);
    vga_print("[KERNEL]  VGA:         ready",   6, 10, COLOR_WHITE);
    vga_print("[KERNEL]  Shell:       ready",   7, 10, COLOR_WHITE);
    vga_print("[KERNEL]  AI layer:    pending", 8, 10, COLOR_WHITE);
    vga_print("Connect via: serial console",   10, 10, COLOR_CYAN);

    shell_run();
}
