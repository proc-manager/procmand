#include <stdio.h>

#include "common/logger.h"
#include "process/process.h"
#include "process/parser.h"


int main(int argc, char* argv[]) {
    struct LogContext ctx;
    get_std_logger(&ctx);

    log_info(&ctx, "some info\n");
    log_debug(&ctx, "some debug\n");
    log_warn(&ctx, "some warn\n");
    log_error(&ctx, "some err\n");

    struct Process* p = calloc(1, sizeof(struct Process));
    parse_process_yaml("process.yaml", p);
    print_parsed_process(p);
    free_process(p);
}