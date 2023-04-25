#include<stdint.h>
#include<stdlib.h>
#include<string.h>

typedef struct version {
    int32_t major;
    int32_t minor;
    int32_t patch;
} version_t;

version_t __API_VERSION = { 0, 1, 0 };

typedef struct static_string {
    char *ptr;
    size_t len;
} static_string_t;

char *name = "Hello from C!";
static_string_t __PLUGIN_NAME;

void lib_init() {
    __PLUGIN_NAME = (static_string_t){name, strlen(name) };
}

void __attribute__ ((constructor)) \
  lib_init();

