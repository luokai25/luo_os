#pragma once
#include <stdint.h>
#include <stddef.h>

#define FS_MAX_FILES    32
#define FS_MAX_FILENAME 32
#define FS_MAX_FILESIZE 4096

typedef struct {
    char    name[FS_MAX_FILENAME];
    uint8_t data[FS_MAX_FILESIZE];
    size_t  size;
    uint8_t used;
} fs_file_t;

void   fs_init(void);
int    fs_create(const char* name);
int    fs_write(const char* name, const char* data, size_t len);
int    fs_read(const char* name, char* buf, size_t max);
int    fs_delete(const char* name);
int    fs_exists(const char* name);
int    fs_append(const char* name, const char* data, size_t len);
size_t fs_size(const char* name);
void   fs_list(void);
