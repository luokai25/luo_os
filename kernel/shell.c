#include "shell.h"
#include "serial.h"
#include "memory.h"
#include "fs.h"
#include "process.h"
#include "timer.h"
#include "io.h"

static int str_eq(const char* a, const char* b) {
    int i = 0;
    while (a[i] && b[i] && a[i]==b[i]) i++;
    return a[i]==b[i];
}

static int str_starts(const char* s, const char* p) {
    int i = 0;
    while (p[i] && s[i]==p[i]) i++;
    return p[i]=='\0';
}

static const char* skip_word(const char* s) {
    while (*s && *s!=' ') s++;
    while (*s==' ') s++;
    return s;
}

static void str_copy(char* dst, const char* src, int max) {
    int i = 0;
    while (i < max-1 && src[i]) { dst[i]=src[i]; i++; }
    dst[i] = '\0';
}

static int str_len(const char* s) {
    int i = 0; while (s[i]) i++; return i;
}

static void cmd_help(void) {
    serial_println("  === LUO_OS v1.0 Commands ===");
    serial_println("");
    serial_println("  [System]");
    serial_println("    help              show this list");
    serial_println("    version           OS version");
    serial_println("    about             about luo_os");
    serial_println("    uptime            system uptime in seconds");
    serial_println("    clear             clear the screen");
    serial_println("    reboot            reboot the system");
    serial_println("    echo <text>       print text to screen");
    serial_println("");
    serial_println("  [Memory]");
    serial_println("    meminfo           show memory usage");
    serial_println("    memtest           run allocator stress test");
    serial_println("");
    serial_println("  [Filesystem]");
    serial_println("    ls                list all files");
    serial_println("    cat <file>        print file contents");
    serial_println("    touch <file>      create empty file");
    serial_println("    rm <file>         delete file");
    serial_println("    write <f> <text>  write text to file");
    serial_println("    append <f> <text> append text to file");
    serial_println("    stat <file>       show file info");
    serial_println("");
    serial_println("  [Processes]");
    serial_println("    ps                list all processes");
    serial_println("    kill <pid>        kill a process by PID");
    serial_println("");
    serial_println("  [AI Agent]");
    serial_println("    ai <cmd>          send command to AI agent");
    serial_println("    agents            list registered AI agents");
}

static void cmd_memtest(void) {
    serial_print("  Running memory allocator test... ");
    void* ptrs[16];
    for (int i = 0; i < 16; i++) {
        ptrs[i] = kmalloc((size_t)(64 * (i+1)));
        if (!ptrs[i]) { serial_println("FAIL (alloc)"); return; }
    }
    for (int i = 0; i < 16; i++) kfree(ptrs[i]);
    void* big = kmalloc(65536);
    if (!big) { serial_println("FAIL (big alloc)"); return; }
    kfree(big);
    serial_println("PASS");
    serial_println("  Allocator: all tests passed.");
}

void shell_run(void) {
    char buf[256];
    char fbuf[FS_MAX_FILESIZE + 1];

    serial_println("luo_os v1.0 ready. Type 'help' for commands.");
    serial_println("");

    while (1) {
        serial_print("luo_os:~$ ");
        serial_readline(buf, 256);
        if (!buf[0]) continue;

        const char* arg  = skip_word(buf);

        if (str_eq(buf, "help")) {
            cmd_help();

        } else if (str_eq(buf, "version")) {
            serial_println("  luo_os v1.0");
            serial_println("  Arch:    x86-32");
            serial_println("  Kernel:  C + ASM");
            serial_println("  Shell:   20+ commands");
            serial_println("  FS:      RAM-based");
            serial_println("  Author:  luokai25");

        } else if (str_eq(buf, "about")) {
            serial_println("  luo_os — desktop OS built from scratch");
            serial_println("  Goal: run humans and AI agents side by side");
            serial_println("  Stack: ASM + C (kernel) | Python (AI layer)");
            serial_println("  Repo: github.com/luokai25/luo_os");

        } else if (str_eq(buf, "uptime")) {
            serial_print("  Uptime: ");
            serial_print_int((int)(timer_ticks() / 100));
            serial_println(" seconds");

        } else if (str_eq(buf, "clear")) {
            serial_print("\033[2J\033[H");

        } else if (str_eq(buf, "reboot")) {
            serial_println("  Rebooting...");
            outb(0x64, 0xFE);

        } else if (str_starts(buf, "echo ")) {
            serial_println(arg);

        } else if (str_eq(buf, "meminfo")) {
            uint32_t used, free_mem, total;
            memory_stats(&used, &free_mem, &total);
            serial_print("  Total heap : "); serial_print_int((int)total);    serial_println(" bytes");
            serial_print("  Used       : "); serial_print_int((int)used);     serial_println(" bytes");
            serial_print("  Free       : "); serial_print_int((int)free_mem); serial_println(" bytes");

        } else if (str_eq(buf, "memtest")) {
            cmd_memtest();

        } else if (str_eq(buf, "ls")) {
            serial_println("  Files in luo_os filesystem:");
            fs_list();

        } else if (str_starts(buf, "cat ")) {
            if (!arg[0]) { serial_println("  Usage: cat <filename>"); continue; }
            int r = fs_read(arg, fbuf, FS_MAX_FILESIZE);
            if (r < 0) {
                serial_print("  Error: no such file: "); serial_println(arg);
            } else {
                serial_print(fbuf);
                if (fbuf[r-1] != '\n') serial_putchar('\n');
            }

        } else if (str_starts(buf, "touch ")) {
            if (!arg[0]) { serial_println("  Usage: touch <filename>"); continue; }
            if (fs_create(arg) == 0) {
                serial_print("  Created: "); serial_println(arg);
            } else {
                serial_println("  Error: file already exists or FS full");
            }

        } else if (str_starts(buf, "rm ")) {
            if (!arg[0]) { serial_println("  Usage: rm <filename>"); continue; }
            if (fs_delete(arg) == 0) {
                serial_print("  Deleted: "); serial_println(arg);
            } else {
                serial_print("  Error: no such file: "); serial_println(arg);
            }

        } else if (str_starts(buf, "stat ")) {
            if (!arg[0]) { serial_println("  Usage: stat <filename>"); continue; }
            if (!fs_exists(arg)) {
                serial_print("  Error: no such file: "); serial_println(arg);
            } else {
                serial_print("  Name: "); serial_println(arg);
                serial_print("  Size: "); serial_print_int((int)fs_size(arg));
                serial_println(" bytes");
            }

        } else if (str_starts(buf, "write ")) {
            if (!arg[0]) { serial_println("  Usage: write <file> <text>"); continue; }
            char fname[32];
            str_copy(fname, arg, 32);
            for (int i = 0; fname[i]; i++) if (fname[i]==' '){fname[i]='\0';break;}
            const char* text = skip_word(arg);
            if (!fs_exists(fname)) fs_create(fname);
            fs_write(fname, text, (size_t)str_len(text));
            serial_print("  Wrote to "); serial_println(fname);

        } else if (str_starts(buf, "append ")) {
            if (!arg[0]) { serial_println("  Usage: append <file> <text>"); continue; }
            char fname[32];
            str_copy(fname, arg, 32);
            for (int i = 0; fname[i]; i++) if (fname[i]==' '){fname[i]='\0';break;}
            const char* text = skip_word(arg);
            if (!fs_exists(fname)) fs_create(fname);
            fs_append(fname, text, (size_t)str_len(text));
            serial_print("  Appended to "); serial_println(fname);

        } else if (str_eq(buf, "ps")) {
            process_list();

        } else if (str_starts(buf, "kill ")) {
            if (!arg[0]) { serial_println("  Usage: kill <pid>"); continue; }
            int pid = 0;
            for (int i = 0; arg[i]>='0' && arg[i]<='9'; i++)
                pid = pid*10 + (arg[i]-'0');
            process_kill(pid);
            serial_print("  Killed PID "); serial_print_int(pid); serial_putchar('\n');

        } else if (str_eq(buf, "agents")) {
            serial_println("  Registered AI agents: none");
            serial_println("  To register: run agent/daemon.py on host");
            serial_println("  Protocol: serial COM1 at 38400 baud");

        } else if (str_starts(buf, "ai ")) {
            serial_print("  [AI] Routing to agent: ");
            serial_println(arg);
            serial_println("  [AI] No agent connected. Start daemon.py first.");

        } else {
            serial_print("  Unknown command: "); serial_println(buf);
            serial_println("  Type 'help' to see all commands.");
        }
    }
}
