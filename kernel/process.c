#include "process.h"
#include "serial.h"

static pcb_t procs[MAX_PROCESSES];
static int   current = 0;

extern void context_switch(regs_t* old, regs_t* new);

static void str_copy(char* dst, const char* src, int max) {
    int i = 0;
    while (i < max-1 && src[i]) { dst[i]=src[i]; i++; }
    dst[i] = '\0';
}

void process_init(void) {
    for (int i = 0; i < MAX_PROCESSES; i++) procs[i].state = PROC_UNUSED;
    procs[0].pid   = 0;
    procs[0].state = PROC_RUNNING;
    procs[0].ticks = 0;
    str_copy(procs[0].name, "kernel", 32);
    current = 0;
}

int process_spawn(void (*entry)(void), const char* name) {
    int slot = -1;
    for (int i = 1; i < MAX_PROCESSES; i++)
        if (procs[i].state == PROC_UNUSED) { slot = i; break; }
    if (slot == -1) return -1;

    pcb_t* p  = &procs[slot];
    p->pid    = slot;
    p->state  = PROC_READY;
    p->ticks  = 0;
    str_copy(p->name, name, 32);

    uint32_t* sp = &p->stack[STACK_SIZE - 1];
    *sp-- = (uint32_t)entry;
    *sp-- = 0; *sp-- = 0; *sp-- = 0; *sp-- = 0;
    p->regs.esp    = (uint32_t)sp;
    p->regs.eflags = 0x202;
    return slot;
}

void process_kill(int pid) {
    if (pid > 0 && pid < MAX_PROCESSES)
        procs[pid].state = PROC_ZOMBIE;
}

int current_pid(void) { return current; }

void schedule(void) {
    int old  = current;
    int next = -1;
    for (int i = 1; i <= MAX_PROCESSES; i++) {
        int c = (current + i) % MAX_PROCESSES;
        if (procs[c].state == PROC_READY ||
            procs[c].state == PROC_RUNNING) {
            next = c; break;
        }
    }
    if (next == -1 || next == old) return;
    procs[old].state  = PROC_READY;
    procs[next].state = PROC_RUNNING;
    procs[next].ticks++;
    current = next;
    context_switch(&procs[old].regs, &procs[next].regs);
}

void yield(void) { schedule(); }

void process_list(void) {
    const char* states[] = {"unused","ready","running","zombie"};
    serial_println("  PID  NAME            STATE    TICKS");
    serial_println("  ---  ----            -----    -----");
    for (int i = 0; i < MAX_PROCESSES; i++) {
        if (procs[i].state == PROC_UNUSED) continue;
        serial_print("  ");
        serial_print_int(procs[i].pid);
        serial_print("    ");
        serial_print(procs[i].name);
        serial_print("  ");
        serial_print(states[procs[i].state]);
        serial_print("  ");
        serial_print_int((int)procs[i].ticks);
        serial_putchar('\n');
    }
}
