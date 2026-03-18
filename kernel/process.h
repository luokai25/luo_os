#pragma once
#include <stdint.h>

#define MAX_PROCESSES 16
#define STACK_SIZE    8192

typedef enum {
    PROC_UNUSED  = 0,
    PROC_READY   = 1,
    PROC_RUNNING = 2,
    PROC_ZOMBIE  = 3
} proc_state_t;

typedef struct {
    uint32_t esp, ebp, ebx, esi, edi, eflags;
} regs_t;

typedef struct {
    int          pid;
    proc_state_t state;
    regs_t       regs;
    uint32_t     stack[STACK_SIZE];
    char         name[32];
    uint32_t     ticks;
} pcb_t;

void process_init(void);
int  process_spawn(void (*entry)(void), const char* name);
void process_kill(int pid);
void schedule(void);
void yield(void);
int  current_pid(void);
void process_list(void);
