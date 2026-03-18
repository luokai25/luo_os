#pragma once
#include <stdint.h>
#include <stddef.h>

void  memory_init(void);
void* kmalloc(size_t size);
void  kfree(void* ptr);
void  memory_stats(uint32_t* used, uint32_t* free_mem, uint32_t* total);
