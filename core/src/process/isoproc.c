#define _GNU_SOURCE
#include "isoproc.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include <sys/mount.h>
#include <sys/wait.h>
#include <sys/types.h>

// custom 
#include "process.h"
#include "helper.h"
#include "common/logger.h"


int isoproc(void* p) {

    // init process 
    // this process sets up the namespace and monitors the child process
    // it exits the namespace once there are no children

    struct LogContext ctx; 
    get_std_logger(&ctx);

    log_info(&ctx, "creating the init process in the new namespace\n");

    struct Process* process = (struct Process*)p;

    if( chdir(process->ContextDir) != 0 ) {
        log_error(&ctx, "error chdir\n");
        graceful_exit(process, "error chdir to context directory\n" ,1);
    }

    prepare_mntns(process);
    overwrite_env(process);

    int status;
    int pid = fork();
    if ( pid == -1 ){
        log_error(&ctx, "error forking");
        graceful_exit(process, "error forking the job process", 1);
    } else if ( pid == 0 ) {
        log_info(&ctx, "executing child");
        execute_job(process);
        log_info(&ctx, "child exec finished");
        return 0;
    } else {
        log_info(&ctx, "monitoring child proc");
        while(1) {
            waitpid(pid, &status, WNOHANG);
            if (WIFEXITED(status)) {
                log_info(&ctx, "child executed successfully");
                graceful_exit(process, "child exited successfully", 1);
            } else if (WIFSIGNALED(status)) {
                log_info(&ctx, "child terminated with signal");
                graceful_exit(process, "error in child", 1);
            }         
            sleep(1);
        } 
    }
    return 0;
}


void prepare_mntns(struct Process* proc) {
    char buffer[150];
    char* mntfs;

    printf("preparing mntns\n");

    if ( sprintf(buffer, "%s/%s", proc->ContextDir, "rootfs") < 0 ) {
        graceful_exit(proc, "error copying rootfs path to buf\n", 1);
    }

    mntfs = strdup(buffer);
    proc->Rootfs = mntfs;

    if ( mount(proc->Rootfs, mntfs, "ext4", MS_BIND, "")) {
        graceful_exit(proc, "error mounting\n", 1);
    } 
    printf("mounted rootfs\n");

    if ( chdir(mntfs) ) {
        graceful_exit(proc, "error chdir\n", 1);
    }
    printf("changed dir to: %s\n", mntfs);

    const char* put_old = ".put_old";
    if( mkdir(put_old, 0777) && errno != EEXIST ) {
        graceful_exit(proc, "error creating the putold directory\n", 1);
    }
    printf("created .put_old\n");

    if ( syscall(SYS_pivot_root, ".", put_old) == -1 ) {  
        graceful_exit(proc, "error pivoting root\n", 1);
    }
    printf("performed sys_pivot\n");

    if ( chdir("/") ) {
        graceful_exit(proc, "error chdir to root\n", 1);
    }
    printf("chdir to root successful\n");

    
    printf("proc initial setup done\n");

}


void overwrite_env(struct Process* proc) {

    printf("overwriting env\n");

    if (proc == NULL || proc->Env == NULL) {
        return;
    }

    if ( clearenv() ) {
        graceful_exit(proc, "error clearenv", 1);
    }

    struct ProcessEnv* env = proc->Env;
    for(int i=0; i< env->count; i++) {
        if( setenv(env->env[i]->Key, env->env[i]->Val, 1) ) {
            graceful_exit(proc, "error setenv", 1);
        }
    }

    printf("env overwrite success");
    
}

void execute_job(struct Process* proc) {


    struct ProcessJob* job = proc->Job;
    struct ProcessJobCommand* cmd = job->Command;
    printf("executing job: %s\n", job->Name);

    if ( execvp(cmd->command, cmd->args) == -1 ) {
        graceful_exit(proc, "execvp failed", 1);
    }

    graceful_exit(proc, "success", 0);

}
