#ifndef ISOLATED_PROCESS_H
#define ISOLATED_PROCESS_H

#include "process.h"

int isoproc(void *p);


void prepare_mntns(struct Process* proc);
void overwrite_env(struct Process* proc);
void execute_job(struct Process* proc);
void prepare_procfs(struct Process* proc);
void prepare_utsns();
void prepare_userns(struct Process* proc);

#endif // ISOLATED_PROCESS_H