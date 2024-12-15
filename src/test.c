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
    info("INFO");
    warning("WARNING");
    error("ERROR");
    success("SUCCESS");
    critical("CRITICAL ERROR");
    debug("DEBUG");
    log_deinit();
    return 0;
}
