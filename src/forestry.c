/*
 *  forestry.c
 *
 *  forestry is a simple logging library for C.
 *  It provides a simple interface for logging messages to the console.
 *  forestry is designed to be simple to use and easy to integrate into existing projects.
 *  It also uses arena allocation to reduce the number of syscalls and increase memory safety.
 *
 *  Copyright 2024 uptu <uptu@uptu.dev>
 */
#include "util/forestry.h"

static uint16_t INDEX = 0x0000;     // Index of the current log message
static char FLAGS = 0b0000;         // Format flags for the current log message
static char *BUF = NULL;            // Memory buffer
static clock_t START = 0;           // Start time for the logged program

const int BUF_SIZE = 1024;
const double CLOCKS_PER_MS = CLOCKS_PER_SEC / 1000;
const char *BUFF_OF_ERROR =
    "Buffer overflow: attempted to write at %d in a buffer of length %d\n";
const char *INDEX_OF_WARN =
    "Log index overflowed; log index may be inaccurate.";

typedef enum {
    INFO,
    WARNING,
    ERROR,
    SUCCESS,
    CRITICAL,
    DEBUG,
} LogLevel;

void set_log_opts(const FormatOptions *opts, unsigned int len) {
    for (unsigned int i = 0; i < len; i++) {
        switch (opts[i]) {
            case NO_INDEX:
                FLAGS |= 0b0001;
                break;
            case NO_SYMBOL:
                FLAGS |= 0b0010;
                break;
            case NO_COLOR:
                FLAGS |= 0b0100;
                break;
            case NO_BOLD:
                FLAGS |= 0b1000;
                break;
            case PLAIN:
                FLAGS |= 0b1100;
                break;
            case BASIC:
                FLAGS |= 0b1111;
                break;
            case TIMER:
                FLAGS |= 0b01000000;
                break;
            case RESET:
                FLAGS &= 0b0000;
                break;
        }
    }
}

void set_log_timer(clock_t start) {
    START = start;
}

void log_deinit() {
    free(BUF);
}

void safe_char_insert(char c, int *i) {
    if (*i < BUF_SIZE) {
        BUF[*i] = c;
        (*i)++;
        return;
    }
    fprintf(stderr, BUFF_OF_ERROR, *i, BUF_SIZE);
    exit(1);
}

int push_fmt(LogLevel lvl, int i) {
    if (0b1100 != (FLAGS & 0b1100)) {
        i += snprintf(BUF + i, 1024 - i, "\x1b[0");
        if (0 == (FLAGS & 0b0100)) {
            switch (lvl) {
                case INFO:
                    i += snprintf(BUF + i, 1024 - i, ";34");
                    break;
                case WARNING:
                    i += snprintf(BUF + i, 1024 - i, ";33");
                    break;
                case ERROR:
                    i += snprintf(BUF + i, 1024 - i, ";31");
                    break;
                case SUCCESS:
                    i += snprintf(BUF + i, 1024 - i, ";32");
                    break;
                case CRITICAL:
                    i += snprintf(BUF + i, 1024 - i, ";37;41");
                    break;
                case DEBUG:
                    i += snprintf(BUF + i, 1024 - i, ";35");
                    break;
            }
        }
        if (0 == (FLAGS & 0b1000)) {
            switch (lvl) {
                case ERROR:
                case SUCCESS:
                case CRITICAL:
                    i += snprintf(BUF + i, 1024 - i, ";1");
                    break;
                default:
                    break;
            }
        }
        i += snprintf(BUF + i, 1024 - i, "m");
    }
    return i;
}

int push_clear(int i) {
    if (0b1100 != (FLAGS & 0b1100)) {
        i += snprintf(BUF + i, 1024 - i, "\x1b[0m");
    }
    return i;
}

// returns the length of the header in the mempage
int fmt_header(LogLevel lvl) {
    int i = 0;
    const char PREFIX_I = '*';
    const char PREFIX_W = '~';
    const char PREFIX_E = '!';
    const char PREFIX_S = '+';
    const char PREFIX_C = '%';
    const char PREFIX_D = '?';
    char *fmt = NULL;

    if (0b0011 == (FLAGS & 0b0011)) {
        return i;
    }

    safe_char_insert('[', &i);

    i = push_fmt(lvl, i);

    if (0 == (FLAGS & 0b0001)) {
        i += snprintf(BUF + i, 1024 - i, "%04x", INDEX);
    }

    i = push_clear(i);

    if (0 == (FLAGS & 0b0011)) {
        i += snprintf(BUF + i, 1024 - i, ":");
    }

    i = push_fmt(lvl, i);

    if (0 == (FLAGS & 0b0010)) {
        switch (lvl) {
            case INFO:
                safe_char_insert(PREFIX_I, &i);
                break;
            case WARNING:
                safe_char_insert(PREFIX_W, &i);
                break;
            case ERROR:
                safe_char_insert(PREFIX_E, &i);
                break;
            case SUCCESS:
                safe_char_insert(PREFIX_S, &i);
                break;
            case CRITICAL:
                safe_char_insert(PREFIX_C, &i);
                break;
            case DEBUG:
                safe_char_insert(PREFIX_D, &i);
                break;
        }
    }
    i = push_clear(i);
    safe_char_insert(']', &i);

    if (0b01000000 == (FLAGS & 0b01000000)) {
        safe_char_insert('(', &i);
        if (0 == START) {
            set_log_timer(clock());
        }
        clock_t elapsed = clock() - START;
        i = push_fmt(lvl, i);
        i += snprintf(
            BUF + i,
            1024 - i,
            "%.3lf",
            (double)elapsed / CLOCKS_PER_MS);
        i = push_clear(i);
        safe_char_insert(')', &i);
    }

    safe_char_insert(' ', &i);
    return i;
}

int fmt_string(LogLevel lvl, char *msg, int i) {
    i = push_fmt(lvl, i);
    i += snprintf(BUF + i, 1024 - i, "%s\x1b[0m\n", msg);
    return i;
}

void log_print(LogLevel lvl, char *msg) {
    if (NULL == BUF) {
        BUF = malloc(BUF_SIZE);
    }

    int i = fmt_header(lvl);
    i += fmt_string(lvl, msg, i);
    fputs(BUF, stderr);

    INDEX++;
    if (0x0000 == INDEX) {
        log_print(WARNING, (char *)INDEX_OF_WARN);
    }
}

void info(char *msg) {
    log_print(INFO, msg);
}

void warning(char *msg) {
    log_print(WARNING, msg);
}

void error(char *msg) {
    log_print(ERROR, msg);
}

void success(char *msg) {
    log_print(SUCCESS, msg);
}

void critical(char *msg) {
    log_print(CRITICAL, msg);
}

void debug(char *msg) {
    log_print(DEBUG, msg);
}
