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

    int clone_flags = SIGCHLD | CLONE_NEWNS;
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


void print_usage() {
    printf("\nUsage: procman <filepath>\n");
}



int main(int argc, char* argv[]) {
    struct LogContext ctx;
    get_std_logger(&ctx);    
    if (argc < 2) {
        print_usage();
        return;
    }
    struct Process* p = calloc(1, sizeof(struct Process));
    start_process(argv[1], p);
    free_process(p);
}