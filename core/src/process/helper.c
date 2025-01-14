#include "helper.h"

#include<stdio.h>
#include<stdlib.h>

#include "process.h"
#include "common/logger.h"


void graceful_exit(struct Process* proc, char* msg, int exit_code) {
    perror(msg);
    struct LogContext log_ctx;
    get_std_logger(&log_ctx);

    log_info(&log_ctx, "graceful exit called\n");

    free_process(proc);
    exit(exit_code);
}