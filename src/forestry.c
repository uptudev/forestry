/*
 *  forestry.c
 *  ^^^^^^^^^^
 *
 *  forestry is a simple logging library for C.
 *  It provides a simple interface for logging messages to the console.
 *  forestry is designed to be simple to use and easy to integrate into existing projects.
 *  It also uses arena allocation to reduce the number of syscalls and increase memory safety.
 *
 *  See LICENSE file for licensing information.
 */

#include "forestry.h"

/* STATE */
// 16-bit index of the current log message
static uint16_t INDEX = 0x0000;
// Start time for the logged program
static clock_t START = 0;
// Format flags for the current log message
static char FLAGS = 0b0000;
// Restricted heap buffer
static char *restrict BUF = NULL;
// Heap buffer pointer
static int i = 0;
// Logfile
static FILE *restrict L_FILE = NULL;

/* CONSTANTS */
// Heap buffer size
const int BUF_SIZE = 16;
// File name max length
const int FILE_NAME_MAX = 32;
// CPU-specific value for division
const double CLOCKS_PER_MS =
    CLOCKS_PER_SEC / 1000;
// Buffer overflow error message
const char *restrict BUFF_OF_ERR =
    "\n\x1b[0mBuffer overflowed twice; make buffer longer or log message shorter.\n";
// Index overflow warn message
const char *restrict INDEX_OF_WARN =
    "Log index overflowed; log index may be inaccurate.";

/*
 * LogLevel
 *
 * Represents the log priority level of a message.
 */
typedef enum {
    INFO,
    WARNING,
    ERROR,
    SUCCESS,
    CRITICAL,
    DEBUG,
} LogLevel;

/*
 * set_log_opts(const LogOptions *restrict opts, unsigned int len)
 *
 * Takes an array of LogOptions and the length of the array.
 * Sets the FLAGS static variable to reflect the desired LogOptions.
 */
void set_log_opt(const LogOptions opt) {
    switch (opt) {
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
        case LOG_FILE:
            FLAGS |= 0b00010000;
            break;
        case ONLY_FILE:
            FLAGS |= 0b00110000;
            break;
        case RESET:
            FLAGS &= 0b0000;
            break;
    }
}

/*
 * set_log_timer(clock_t start)
 *
 * Takes a clock_t (from time.h) representing the start of the tracked runtime.
 * If this is not set, the logging time will begin at the start of the first log call.
 */
void set_log_timer(clock_t start) {
    START = start;
    FLAGS |= 0b01000000;
}

/*
 * set_log_file(FILE *restrict file)
 *
 * Takes a restricted FILE reference to the desired path.
 */
void set_log_file(FILE *restrict file) {
    L_FILE = file;
    FLAGS |= 0b00010000;
}

/*
 * log_deinit()
 *
 * Frees the static buffer at the end of the runtime.
 * This must be ran after logging is done or 1024B of memory will leak.
 */
void log_deinit() {
    if (0 == (FLAGS & 0b00100000)) fputs(BUF, stderr);
    if ((0 != (FLAGS & 0b00010000)) && (NULL != L_FILE)) {
        fputs(BUF, L_FILE);
    }
    if (NULL != L_FILE) fclose(L_FILE);
    free(BUF);
}

FILE *gen_log_file() {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC_RAW, &ts);
    uint64_t us = ((uint64_t)ts.tv_sec * 1000000) + ((uint64_t)ts.tv_nsec / 1000);
    char filename[FILE_NAME_MAX];
    snprintf(filename, FILE_NAME_MAX, "%x.log", us);
    FILE *file = fopen(filename, "w");
    return file;
}

/*
 * flush_buf(int *restrict i)
 *
 * flushes the buffer and resets the buffer pointer
 */
void flush_buf(int *restrict i) {
    if (0 == (FLAGS & 0b00100000)) fputs(BUF, stderr);
    if (0 != (FLAGS & 0b00010000)) {
        if (NULL == L_FILE) {
            L_FILE = gen_log_file();
        }
        fputs(BUF, L_FILE);
    }
    *i = 0;
}

/*
 * buf_fmt_double(int *restrict i, const char *restrict fmt, double msg)
 *
 * Safely adds text to the buffer, flushing it in case of an overflow.
 * If it still overflows after flushing, it gracefully exits.
 */
void buf_fmt_double(
        int *restrict i,
        const char *restrict fmt,
        double msg
        ) {
    int written = snprintf(BUF + *i, BUF_SIZE - *i, fmt, msg);
    int rem = written - (BUF_SIZE - *i);
    if (0 <= rem) {
        BUF[*i] = '\0';
        flush_buf(i);
        written = snprintf(BUF + *i, BUF_SIZE - *i, fmt, msg);
        rem = written - (BUF_SIZE - *i);
        if (0 <= rem) {
            fputs(BUFF_OF_ERR, stderr);
        } else {
            *i += written;
        }
    } else {
        *i += written;
    }
}

/*
 * buf_fmt_short(int *restrict i, const char *restrict fmt, uint16_t msg)
 *
 * Safely adds text to the buffer, flushing it in case of an overflow.
 * If it still overflows after flushing, it gracefully exits.
 */
void buf_fmt_short(
        int *restrict i,
        const char *restrict fmt,
        uint16_t msg
        ) {
    int written = snprintf(BUF + *i, BUF_SIZE - *i, fmt, msg);
    int rem = written - (BUF_SIZE - *i);
    if (0 <= rem) {
        BUF[*i] = '\0';
        flush_buf(i);
        written = snprintf(BUF + *i, BUF_SIZE - *i, fmt, msg);
        rem = written - (BUF_SIZE - *i);
        if (0 <= rem) {
            fputs(BUFF_OF_ERR, stderr);
        } else {
            *i += written;
        }
    } else {
        *i += written;
    }
}

/*
 * buf_print(int *restrict i, char *restrict msg)
 *
 * Safely adds text to the buffer, flushing it in case of an overflow.
 */
void buf_print(int *restrict i, char *restrict msg) {
    int written = snprintf(BUF + *i, BUF_SIZE - *i, "%s", msg);
    int rem = written - (BUF_SIZE - *i);
    // if buffer overflows, flush to stderr and reset `i`
    if (0 <= rem) {
        BUF[*i] = '\0';
        flush_buf(i);
        written = snprintf(BUF + *i, BUF_SIZE - *i, "%s", msg);
        rem = written - (BUF_SIZE - *i);
        if (0 <= rem) {
            fputs(BUFF_OF_ERR, stderr);
            return;
        }
    }
    *i += written;
}

/*
 * safe_char_insert(char c, int *restrict i)
 *
 * Safely inserts a char into the buffer, incrementing the buffer pointer.
 * If the pointer overflows the buffer, the buffer is flushed and attempted again.
 */
void safe_char_insert(char c, int *restrict i) {
    if (*i < BUF_SIZE) {
        BUF[*i] = c;
        (*i)++;
        return;
    }
    flush_buf(i);
    safe_char_insert(c, i);
}

/*
 * push_clear(int *restrict i)
 *
 * Generates an ANSI escape RESET sequence to clear any formatting after the code.
 * This is then pushed to the buffer, and the buffer pointer is incremented.
 */
void push_clear(int *restrict i) {
    if (0b1100 != (FLAGS & 0b1100)) {
        buf_print(i, "\x1b[0m");
    }
}

/*
 * push_fmt(LogLevel lvl, int *restric i)
 *
 * Generates an ANSI escape seqeunce based on the static FLAGS.
 * This sequence is then pushed to the buffer, and the pointer is incremented.
 */
void push_fmt(LogLevel lvl, int *restrict i) {
    push_clear(i);
    if (0 == (FLAGS & 0b0100)) {
        switch (lvl) {
            case INFO:
                buf_print(i, "\x1b[34m");
                break;
            case WARNING:
                buf_print(i, "\x1b[33m");
                break;
            case ERROR:
                buf_print(i, "\x1b[31m");
                break;
            case SUCCESS:
                buf_print(i, "\x1b[32m");
                break;
            case CRITICAL:
                buf_print(i, "\x1b[37;41m");
                break;
            case DEBUG:
                buf_print(i, "\x1b[35m");
                break;
        }
    }
    if (0 == (FLAGS & 0b1000)) {
        switch (lvl) {
            case ERROR:
            case SUCCESS:
            case CRITICAL:
                buf_print(i, "\x1b[1m");
                break;
            default:
                break;
        }
    }
}

/*
 * fmt_header(LogLevel lvl) -> int
 *
 * Pushes a generated prefix to the log message to the buffer.
 * The buffer pointer's value is set to the amount of bytes written.
 * Since this will always be the first function called by log_print,
 * i is not a restricted pointer argument but is the return value.
 */
void fmt_header(LogLevel lvl, int *restrict i) {
    const char PREFIX_I = '*';
    const char PREFIX_W = '~';
    const char PREFIX_E = '!';
    const char PREFIX_S = '+';
    const char PREFIX_C = '%';
    const char PREFIX_D = '?';
    char *fmt = NULL;

    if (0b0011 == (FLAGS & 0b0011)) {
        return;
    }

    safe_char_insert('[', i);

    if (0 == (FLAGS & 0b0001)) {
        push_fmt(lvl, i);
        buf_fmt_short(i, "%04x", INDEX);
        push_clear(i);
    }

    if (0 == (FLAGS & 0b0011)) {
        buf_print(i, ":");
    }

    if (0 == (FLAGS & 0b0010)) {
        push_fmt(lvl, i);
        switch (lvl) {
            case INFO:
                safe_char_insert(PREFIX_I, i);
                break;
            case WARNING:
                safe_char_insert(PREFIX_W, i);
                break;
            case ERROR:
                safe_char_insert(PREFIX_E, i);
                break;
            case SUCCESS:
                safe_char_insert(PREFIX_S, i);
                break;
            case CRITICAL:
                safe_char_insert(PREFIX_C, i);
                break;
            case DEBUG:
                safe_char_insert(PREFIX_D, i);
                break;
        }
        push_clear(i);
    }

    safe_char_insert(']', i);

    if (0b01000000 == (FLAGS & 0b01000000)) {
        safe_char_insert('(', i);
        if (0 == START) {
            set_log_timer(clock());
        }
        clock_t elapsed = clock() - START;
        push_fmt(lvl, i);
        buf_fmt_double(i, "%.3lfms", (double)elapsed / CLOCKS_PER_MS);
        push_clear(i);
        safe_char_insert(')', i);
    }

    safe_char_insert(' ', i);
}

/*
 * fmt_string(LogLevel lvl, char *restrict msg, int *restrict i)
 *
 * Pushes the formatted string to the buffer, incrementing the buffer pointer.
 */
void fmt_string(LogLevel lvl, char *restrict msg, int *restrict i) {
    push_fmt(lvl, i);
    buf_print(i, msg);
    push_clear(i);
    buf_print(i, "\n");
}

/*
 * log_print(LogLevel lvl, char *restrict msg)
 *
 * Prints a message to stderr
 */
void log_print(LogLevel lvl, char *restrict msg) {
    if (NULL == BUF) {
        BUF = malloc(BUF_SIZE);
    }

    fmt_header(lvl, &i);
    fmt_string(lvl, msg, &i);

    INDEX++;
    if (0x0000 == INDEX) {
        log_print(WARNING, (char *restrict)INDEX_OF_WARN);
    }
}

/*
 * info(char *restrict msg)
 *
 * Logs an INFO message to stderr.
 */
void log_info(char *restrict msg) {
    log_print(INFO, msg);
}

/*
 * void warning(char *restrict msg)
 *
 * Logs a WARN message to stderr.
 */
void log_warning(char *restrict msg) {
    log_print(WARNING, msg);
}

/*
 * error(char *restrict msg)
 *
 * Logs an ERROR message to stderr.
 */
void log_error(char *restrict msg) {
    log_print(ERROR, msg);
}

/*
 * success(char *restrict msg)
 *
 * Logs a SUCCESS message to stderr.
 */
void log_success(char *restrict msg) {
    log_print(SUCCESS, msg);
}

/*
 * critical(char *restrict msg)
 *
 * Logs a CRITICAL error message to stderr.
 */
void log_critical(char *restrict msg) {
    log_print(CRITICAL, msg);
}

/*
 * debug(char *restrict msg)
 *
 * Logs a DEBUG message to stderr.
 */
void log_debug(char *restrict msg) {
    log_print(DEBUG, msg);
}
