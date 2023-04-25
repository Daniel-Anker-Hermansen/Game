#include<stdint.h>
#include<stdlib.h>
#include<string.h>

typedef struct version {
    int32_t major;
    int32_t minor;
    int32_t patch;
} version_t;

version_t __api_version() {
    version_t version;
    version.major = 0;
    version.minor = 1;
    version.patch = 0;
    return version;
}

typedef struct static_string {
    char *ptr;
    size_t len;
} static_string_t;

static_string_t __plugin_name() {
    char *name = "Hello from C!";
    static_string_t string;
    string.ptr = name;
    string.len = strlen(name);
    return string;
}
