#ifndef _FORESTRY_H
#define _FORESTRY_H
#define _POSIX_C_SOURCE 199309L
#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#include <inttypes.h>
typedef enum {
    NO_INDEX,
    NO_SYMBOL,
    NO_COLOR,
    NO_BOLD,
    PLAIN,
    BASIC,
    TIMER,
    LOG_FILE,
    ONLY_FILE,
    RESET,
} LogOptions;
void set_log_opt(const LogOptions opt);
void set_log_timer(clock_t start);
void set_log_file(FILE *restrict file);
void log_info(char *restrict msg);
void log_warning(char *restrict msg);
void log_error(char *restrict msg);
void log_success(char *restrict msg);
void log_critical(char *restrict msg);
void log_debug(char *restrict msg);
void log_deinit();
#endif  // _FORESTRY_H
