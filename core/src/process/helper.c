#include "helper.h"

#include<stdio.h>
#include<stdlib.h>

#include "process.h"
#include "common/logger.h"

void graceful_exit(struct Process* proc, char* msg, int exit_code) {
    printf("graceful exit called");

    free_process(proc);
    perror(msg);
    exit(exit_code);
}