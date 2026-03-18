#pragma once
#include <stdint.h>
void serial_init(void);
void serial_putchar(char c);
void serial_print(const char* s);
char serial_getchar(void);
uint8_t serial_received(void);
