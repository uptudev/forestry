#ifndef _FORESTRY_H
#define _FORESTRY_H
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <inttypes.h>
typedef enum {
    NO_INDEX,
    NO_SYMBOL,
    NO_COLOR,
    NO_BOLD,
    TIMER,
    PLAIN,
    BASIC,
    RESET,
} FormatOptions;
void set_log_opts(const FormatOptions *opts, unsigned int len);
void set_log_timer(clock_t start);
void info(char *msg);
void warning(char *msg);
void error(char *msg);
void success(char *msg);
void critical(char *msg);
void debug(char *msg);
void log_deinit();
#endif  // _FORESTRY_H
