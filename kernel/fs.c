#include "fs.h"
#include "serial.h"

static fs_file_t files[FS_MAX_FILES];

static int str_eq(const char* a, const char* b) {
    int i = 0;
    while (a[i] && b[i] && a[i]==b[i]) i++;
    return a[i]==b[i];
}

static void str_copy(char* dst, const char* src, int max) {
    int i = 0;
    while (i < max-1 && src[i]) { dst[i]=src[i]; i++; }
    dst[i] = '\0';
}

static fs_file_t* find(const char* name) {
    for (int i = 0; i < FS_MAX_FILES; i++)
        if (files[i].used && str_eq(files[i].name, name))
            return &files[i];
    return 0;
}

void fs_init(void) {
    for (int i = 0; i < FS_MAX_FILES; i++) files[i].used = 0;
    fs_create("readme.txt");
    fs_write("readme.txt",
        "Welcome to luo_os v1.0\n"
        "Built from scratch by luokai25\n"
        "Stack: ASM + C + Python\n"
        "Goal: Human + AI Desktop OS\n", 91);
    fs_create("motd.txt");
    fs_write("motd.txt",
        "LUO_OS: No limits. No boundaries.\n", 35);
}

int fs_create(const char* name) {
    if (find(name)) return -1;
    for (int i = 0; i < FS_MAX_FILES; i++) {
        if (!files[i].used) {
            files[i].used = 1;
            files[i].size = 0;
            str_copy(files[i].name, name, FS_MAX_FILENAME);
            return 0;
        }
    }
    return -1;
}

int fs_write(const char* name, const char* data, size_t len) {
    fs_file_t* f = find(name);
    if (!f) return -1;
    if (len > FS_MAX_FILESIZE) len = FS_MAX_FILESIZE;
    for (size_t i = 0; i < len; i++) f->data[i] = (uint8_t)data[i];
    f->size = len;
    return (int)len;
}

int fs_append(const char* name, const char* data, size_t len) {
    fs_file_t* f = find(name);
    if (!f) return -1;
    size_t space = FS_MAX_FILESIZE - f->size;
    if (len > space) len = space;
    for (size_t i = 0; i < len; i++) f->data[f->size+i] = (uint8_t)data[i];
    f->size += len;
    return (int)len;
}

int fs_read(const char* name, char* buf, size_t max) {
    fs_file_t* f = find(name);
    if (!f) return -1;
    size_t n = f->size < max ? f->size : max;
    for (size_t i = 0; i < n; i++) buf[i] = (char)f->data[i];
    buf[n] = '\0';
    return (int)n;
}

int fs_delete(const char* name) {
    fs_file_t* f = find(name);
    if (!f) return -1;
    f->used = 0;
    return 0;
}

int fs_exists(const char* name) { return find(name) ? 1 : 0; }

size_t fs_size(const char* name) {
    fs_file_t* f = find(name);
    return f ? f->size : 0;
}

void fs_list(void) {
    int count = 0;
    for (int i = 0; i < FS_MAX_FILES; i++) {
        if (files[i].used) {
            serial_print("  ");
            serial_print(files[i].name);
            serial_print("  (");
            serial_print_int((int)files[i].size);
            serial_println(" bytes)");
            count++;
        }
    }
    if (count == 0) serial_println("  (empty)");
}
