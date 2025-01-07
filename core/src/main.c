#define _GNU_SOURCE
#include <stdio.h>

#include<signal.h>
#include<stdlib.h>
#include<stdio.h>
#include<unistd.h>
#include<linux/unistd.h>
#include<linux/sched.h>
#include<sched.h>
#include<sys/wait.h>
#include<sys/syscall.h>

#include "common/logger.h"
#include "process/process.h"
#include "process/parser.h"
#include "process/isoproc.h"
#include "process/helper.h"


void start_process(char* process_yaml_loc, struct Process* p) {
    parse_process_yaml(process_yaml_loc, p);
    
    if ( chdir(p->ContextDir) != 0 ) {
        perror("error changing dir");
        exit(1);
    }

    int clone_flags = SIGCHLD | CLONE_NEWNS | CLONE_NEWUTS | CLONE_NEWUSER;
    char* cmd_stack = malloc(STACKSIZE);

    pid_t pid = clone(isoproc, cmd_stack + STACKSIZE, clone_flags, (void*)p);
    if (pid == -1){
        perror("clone");
        free(cmd_stack);
        exit(EXIT_FAILURE);
    }

    p->Pid = pid;
    p->Stack = cmd_stack;

    if( waitpid(pid, NULL, 0) == -1 ) {
        graceful_exit(p, "waitpid failed", 1);
    }

    graceful_exit(p, "success", 0);
}

int main() {
    struct LogContext ctx;
    get_std_logger(&ctx);

    log_info(&ctx, "some info\n");
    log_debug(&ctx, "some debug\n");
    log_warn(&ctx, "some warn\n");
    log_error(&ctx, "some err\n");

    // struct Process* p = calloc(1, sizeof(struct Process));
    // parse_process_yaml("process.yaml", p);
    // print_parsed_process(p);
    // free_process(p);
}