#include <stdio.h>

#include "common/logger.h"

int main() {
    struct LogContext ctx;
    get_std_logger(&ctx);

    log_info(&ctx, "some info\n");
    log_debug(&ctx, "some debug\n");
    log_warn(&ctx, "some warn\n");
    log_error(&ctx, "some err\n");
}