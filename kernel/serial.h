#pragma once
#include <stdint.h>
void    serial_init(void);
void    serial_putchar(char c);
void    serial_print(const char* s);
void    serial_println(const char* s);
void    serial_print_int(int n);
void    serial_print_hex(uint32_t n);
int     serial_readline(char* buf, int max);
uint8_t serial_received(void);
char    serial_getchar(void);
