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


static int pivot_root(const char* new_root, const char* put_old) {
    return syscall(SYS_pivot_root, new_root, put_old);
}


static int write_file(char path[100], char line[100])
{
    FILE *f = fopen(path, "w");

    if (f == NULL) {
        return -1;         
    }

    if (fwrite(line, 1, strlen(line), f) < 0) {
        return -1;
    }

    if (fclose(f) != 0) {
        return -1;
    }
    return 0;
}


static void await_setup(struct Process* p) {
    char buf[2];
    if( read(p->fd[0], buf, 2) != 2 ) {
        graceful_exit(p, "error await setup\n", 1);
    }
}


int isoproc(void* p) {

    // init process 
    // this process sets up the namespace and monitors the child process
    // it exits the namespace once there are no children

    struct LogContext ctx; 
    get_std_logger(&ctx);

    log_info(&ctx, "creating the init process in the new namespace\n");

    struct Process* process = (struct Process*)p;

    prepare_mntns(process);
    overwrite_env(process);
    prepare_utsns();

    // signal the parent that mnt,proc,env,uts setup is done
    if(write(process->fd[1], "OK", 2) != 2) {
        log_error(&ctx, "error writing to pipe\n");
        graceful_exit(process, "error writing to pipe\n", 1);
    }

    // now wait for user and net namespaces to be created
    if(read(process->fd[0], "OK", 2) != 2) {
        log_error(&ctx, "error reading from pipe\n");
        graceful_exit(process, "error reading from pipe\n", 1);
    }

    int status;
    int pid = fork();
    if ( pid == -1 ){
        log_error(&ctx, "error forking\n");
        graceful_exit(process, "error forking the job process\n", 1);
    } else if ( pid == 0 ) {
        log_info(&ctx, "executing child\n");
        await_setup(p);
        execute_job(process);
        log_info(&ctx, "child exec finished\n"); 
        return 0;
    } else {
        log_info(&ctx, "monitoring child proc\n");

        while(1) {
            sleep(1);
            waitpid(pid, &status, 0);
            if (WIFEXITED(status)) {
                log_info(&ctx, "child executed successfully\n");
                graceful_exit(process, "child exited successfully\n", 1);
            } else if (WIFSIGNALED(status)) {
                log_info(&ctx, "child terminated with signal\n");
                graceful_exit(process, "error in child\n", 1);
            }         
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
    memset(buffer, 0, sizeof(buffer));

    if (mount(NULL, "/", NULL, MS_REC | MS_PRIVATE, NULL) == -1) {
        graceful_exit(proc, "err shared propagation\n", 1);
    }

    if ( mount(proc->Rootfs, proc->Rootfs, "ext4", MS_BIND, "") == -1) {
        graceful_exit(proc, "error mounting - ms_bind\n", 1);
    } 
    printf("mounted rootfs\n");

    if ( chdir(proc->ContextDir) ) {
        graceful_exit(proc, "error chdir", 1);
    }
    printf("changed dir to: %s\n", proc->ContextDir);

    if( sprintf(buffer, "%s/%s", proc->Rootfs, ".put_old") < 0 ) {
        graceful_exit(proc, "error copying put_old to buf\n", 1);
    }
    char* put_old = strdup(buffer);
    memset(buffer, 0, sizeof(buffer));
    if( mkdir(put_old, 0777) == -1 ) {
        graceful_exit(proc, "error creating the putold directory\n", 1);
    }
    printf("created %s\n", put_old);

    printf("\ncalling pivot root with: %s, %s\n", proc->Rootfs, put_old);
    if ( pivot_root(proc->Rootfs, put_old) == -1 ) {  
        free(put_old);
        graceful_exit(proc, "error pivoting root\n", 1);
    }
    printf("performed sys_pivot\n");

    if ( chdir("/") == -1 ) {
        free(put_old);
        graceful_exit(proc, "error chdir to root\n", 1);
    }
    printf("chdir to root successful\n");

    prepare_procfs(proc);

    if(umount2(".put_old", MNT_DETACH) == -1) {
        free(put_old);
        graceful_exit(proc, "failed to umount put_old\n", 1);
    }

    if (rmdir(".put_old") == -1) {
        free(put_old);
        graceful_exit(proc, "rmdir\n", 1);
    }

    free(put_old); 
    printf("proc initial setup done\n");

}


void overwrite_env(struct Process* proc) {

    printf("overwriting env\n");

    if (proc == NULL || proc->Env == NULL) {
        return;
    }

    if ( clearenv() ) {
        graceful_exit(proc, "error clearenv\n", 1);
    }

    struct ProcessEnv* env = proc->Env;
    for(int i=0; i< env->count; i++) {
        if( setenv(env->env[i]->Key, env->env[i]->Val, 1) ) {
            graceful_exit(proc, "error setenv\n", 1);
        }
    }

    printf("env overwrite success\n");
    
}


void execute_job(struct Process* proc) {

    struct ProcessJob* job = proc->Job;
    struct ProcessJobCommand* cmd = job->Command;
    printf("executing job: %s\n", job->Name);

    if ( execvp(cmd->command, cmd->args) == -1 ) {
        graceful_exit(proc, "execvp failed\n", 1);
    }

    graceful_exit(proc, "success\n", 0);

}

void prepare_procfs(struct Process* proc) { 
    // should be executed inside the child 
    if( mkdir("/proc", 0555) == -1 && errno != EEXIST ) {
        graceful_exit(proc, "err mkdir proc\n", 1);
    }

    if( mount("proc", "/proc", "proc", 0, "") == -1 ) {
        graceful_exit(proc, "err mount\n", 1);
    }
}


void prepare_utsns() {
    sethostname("isoproc", strlen("isoproc"));
}



void prepare_userns(struct Process* proc) {
    struct LogContext ctx; 
    get_std_logger(&ctx); 

    log_info(&ctx, "creating userns\n");

    char path[250];
    char line[100];

    int uid = 1000; // let's just go with this

    sprintf(path, "/proc/%d/uid_map", proc->Pid);
    sprintf(line, "0 %d 1\n", uid);
    if(write_file(path, line) != 0) {
        log_error(&ctx, "error writing to proc uid map\n");
        graceful_exit(proc, "error writing to proc uid map\n", 1);
    }

    sprintf(path, "/proc/%d/setgroups", proc->Pid);
    sprintf(line, "deny");
    if(write_file(path, line) != 0) {
        log_error(&ctx, "error writing to proc uid map\n");
        graceful_exit(proc, "error writing to proc uid map\n", 1);
    }

    sprintf(path, "/proc/%d/gid_map", proc->Pid);
    sprintf(line, "0 %d 1\n", uid);
    if(write_file(path, line) != 0) {
        log_error(&ctx, "error writing to proc uid map\n");
        graceful_exit(proc, "error writing to proc uid map\n", 1);
    }
    
}