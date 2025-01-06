#include "process.h"

#include<stdlib.h>
#include<stdio.h>


void free_env(struct Env* env) {
    if ( env == NULL ) {
        return;
    }

    if ( env->Key != NULL ){ 
        free(env->Key);
        env->Key = NULL;
    }

    if ( env->Val != NULL ){
        free(env->Val);
        env->Val = NULL;
    }

    free(env);
    env = NULL;
}

void free_process_env(struct ProcessEnv* penv) {
    if ( penv == NULL ) {
        return;
    } 
    struct Env** env = penv->env;

    if ( env == NULL ){ 
        goto free_penv;
    }
    
    for(int i=0; i < penv->count; i++){
        free_env(env[i]);
    }
    free(env);

    free_penv: 
        free(penv);

    penv = NULL;
}


void free_image(struct Image* image) {
    if ( image == NULL ) {
        return; 
    }

    if ( image-> Id != NULL ) {
        free(image->Id);
        image->Id = NULL;
    }

    if ( image->Name != NULL ) {
        free(image->Name);
        image->Name = NULL;
    }

    if ( image->ContextTempDir != NULL ) {
        free(image->ContextTempDir);
        image->ContextTempDir = NULL;
    }

    if ( image->ImgPath != NULL ) {
        free(image->ImgPath);
        image->ImgPath = NULL;
    }

    if ( image->Tag != NULL ) {
        free(image->Tag);
        image->Tag = NULL;
    }

    if ( image->Created != NULL ) {
        free(image->Created);
        image->Created = NULL;
    }

    free(image);
    image = NULL;
}

void free_port_map(struct PortMap* m) {

    if (m == NULL) {
        return;
    }
    if (m->HostPort != NULL) {
        free(m->HostPort);
    } 
    if (m->ProcPort != NULL) {
        free(m->ProcPort);
    }

    free(m);
    m = NULL;
}


void free_network_port_mapping(struct PortMapping* pm) {
    if( pm == NULL ) {
        return;
    }

    if (pm->pmap == NULL) {
        goto free_pm; 
    }

    for(int i=0; i < pm->nports; i++) {
        free_port_map(pm->pmap[i]);
    }
    free(pm->pmap);      
    pm->pmap = NULL;

    free_pm:
        free(pm);
    
    pm = NULL;
}

void free_process_network_namespace(struct ProcessNetworkNamespace *pns) {
    if( pns != NULL ){
        free(pns);
        pns = NULL;
    }
    return;
}

void free_process_network(struct ProcessNetwork* net) {
    if(net == NULL){
        return;
    }
    free_network_port_mapping(net->pm);
    free(net);
    net = NULL;
}


void free_process_job_command(struct ProcessJobCommand *cmd) {

    if ( cmd == NULL ) {
        return;
    } 

    for(int c=0; c < cmd->argc; c++){
        if( cmd->args[c] != NULL ){
            free(cmd->args[c]);
        }
    }

    if ( cmd->command != NULL ) {
        free(cmd->command);
    }

    if ( cmd->args != NULL ) {
        free(cmd->args);
    }

    free(cmd);
    cmd = NULL;

}

void free_process_job(struct ProcessJob* job) {

    if( job == NULL ){
        return;
    }
    
    if ( job->Name != NULL ) {
        free(job->Name);
    }

    if( job->Command != NULL ) {
        free_process_job_command(job->Command);
    }

    free(job);
    job = NULL;
}

void free_process_stack(struct Process* proc) {
    if ( proc->Stack != NULL ) {
        free(proc->Stack);
    }
}

void free_process_rootfs(struct Process* proc) {
    if( proc->Rootfs != NULL ) {
        free(proc->Rootfs);
    }
}

void free_process(struct Process* process) {
    if( process == NULL ) {
        return;
    }

    if( process->Id != NULL ) {
        free(process->Id);
        process->Id = NULL;
    }

    if (process->Name != NULL ) {
        free(process->Name);
        process->Name = NULL;
    }

    if (process->ContextDir != NULL ) {
        free(process->ContextDir);
        process->ContextDir = NULL;
    }

    free_image(process->Image);
    free_process_job(process->Job);
    free_process_env(process->Env);
    free_process_network(process->Network);
    free_process_stack(process);
    free_process_rootfs(process);

    // free process mem
    free(process);
    process = NULL;
}


