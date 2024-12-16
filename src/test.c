/*
 *  ctr
 *  
 *  ctr is a quick C rewrite of the GNU coreutils tr command.
 *  
 *  Copyright 2024 uptu <uptu@uptu.dev>
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
