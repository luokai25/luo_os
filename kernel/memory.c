#include "memory.h"

#define HEAP_SIZE   (1024 * 1024)
#define BLOCK_MAGIC  0xDEADBEEF

static uint8_t heap[HEAP_SIZE];

typedef struct block {
    uint32_t      magic;
    size_t        size;
    uint8_t       used;
    struct block* next;
    struct block* prev;
} block_t;

static block_t* head = 0;

void memory_init(void) {
    head = (block_t*)heap;
    head->magic = BLOCK_MAGIC;
    head->size  = HEAP_SIZE - sizeof(block_t);
    head->used  = 0;
    head->next  = 0;
    head->prev  = 0;
}

void* kmalloc(size_t size) {
    if (size == 0) return 0;
    size = (size + 7) & ~7u;
    block_t* b = head;
    while (b) {
        if (!b->used && b->size >= size) {
            if (b->size >= size + sizeof(block_t) + 8) {
                block_t* nb = (block_t*)((uint8_t*)b + sizeof(block_t) + size);
                nb->magic = BLOCK_MAGIC;
                nb->size  = b->size - size - sizeof(block_t);
                nb->used  = 0;
                nb->next  = b->next;
                nb->prev  = b;
                if (b->next) b->next->prev = nb;
                b->next = nb;
                b->size = size;
            }
            b->used = 1;
            return (void*)((uint8_t*)b + sizeof(block_t));
        }
        b = b->next;
    }
    return 0;
}

void kfree(void* ptr) {
    if (!ptr) return;
    block_t* b = (block_t*)((uint8_t*)ptr - sizeof(block_t));
    if (b->magic != BLOCK_MAGIC) return;
    b->used = 0;
    if (b->next && !b->next->used) {
        b->size += sizeof(block_t) + b->next->size;
        b->next  = b->next->next;
        if (b->next) b->next->prev = b;
    }
    if (b->prev && !b->prev->used) {
        b->prev->size += sizeof(block_t) + b->size;
        b->prev->next  = b->next;
        if (b->next) b->next->prev = b->prev;
    }
}

void memory_stats(uint32_t* used, uint32_t* free_mem, uint32_t* total) {
    *used = 0; *free_mem = 0; *total = HEAP_SIZE;
    block_t* b = head;
    while (b) {
        if (b->used) *used     += (uint32_t)b->size;
        else          *free_mem += (uint32_t)b->size;
        b = b->next;
    }
}
