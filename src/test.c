/*
 *  test.c
 *  ^^^^^^
 *  This is a simple test program for forestry.
 *  
 *  See LICENSE file for licensing information.
 */

#include "forestry.h"

int main() {
    set_log_opt(ONLY_FILE);
    set_log_opt(PLAIN);
    log_info("INFO");
    log_warning("WARNING");
    log_error("ERROR");
    log_success("SUCCESS");
    log_critical("CRITICAL ERROR");
    log_debug("DEBUG");
    log_deinit();
    return 0;
}
