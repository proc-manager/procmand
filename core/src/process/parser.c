#include "parser.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <yaml.h>

#include "helper.h"
#include "process.h"


void print_parsed_image(struct Image* image) {
    printf("\n-------Image-----\n");
    printf("%s\n", image->Id);
    printf("%s\n", image->Name);
    printf("%s\n", image->ContextTempDir);
    printf("%s\n", image->ImgPath);
    printf("%s\n", image->Tag);
    printf("%s\n", image->Created);
    printf("\n-------Image-----\n");
}

void print_parsed_job(struct ProcessJob* job) {
    printf("\n-------Job-----\n");
    printf("%s\n", job->Name);

    struct ProcessJobCommand* cmd = job->Command;
    printf("command: %s\n", cmd->command);
    for(int c=0; c < cmd->argc; c++){
        if( cmd->args[c] != NULL ){
            printf("%s ", cmd->args[c]);
        }
    }
    printf("\n-------Job-----\n");   
}

void print_process_env(struct ProcessEnv* penv) {
    printf("\n-------ENV-----\n");   
    printf("env_count: %d\n", penv->count);
    struct Env** envs = penv->env;
    for(int e=0; e < penv->count; e++){
        if( penv->env[e] != NULL ){
            printf("key: %s, val: %s\n", envs[e]->Key, envs[e]->Val);
        }
    }
    printf("\n-------ENV-----\n");   
}


void print_process_network(struct ProcessNetwork* net) {
    printf("\n-------NET-----\n");   
    struct PortMapping* pm = net->pm;
    for(int p=0; p< pm->nports; p++) {
        printf("%s:%s\n", pm->pmap[p]->HostPort, pm->pmap[p]->ProcPort);
    }
    printf("\n-------NET-----\n");   
}


void print_parsed_process(struct Process *process) {
    printf("Id = %s\n", process->Id);
    printf("Name = %s\n", process->Name);
    printf("Pid = %d\n", process->Pid);
    print_parsed_image(process->Image);
    print_parsed_job(process->Job);
    print_process_env(process->Env);
    print_process_network(process->Network);
}



void parse_process_yaml(char* filepath, struct Process* process) {

    if (filepath == NULL){
        perror("empty filepath");
        exit(1);
    }

    if (process == NULL){
        perror("process is null");
        exit(1);
    }

    FILE *file = fopen(filepath, "r");
    if (!file) {
        perror("unable to open yaml file");
        exit(1);
    }

    yaml_parser_t parser;
    yaml_event_t event;

    if (!yaml_parser_initialize(&parser)) {
        fprintf(stderr, "failed to initialize yaml parser\n");
        fclose(file);
        exit(1);
    }
    yaml_parser_set_input_file(&parser, file);

    char* key = NULL;

    while(1) {
        yaml_parser_parse(&parser, &event);
        if(event.type == YAML_STREAM_END_EVENT) {
            yaml_event_delete(&event);
            break;
        }

        switch(event.type) {
            case YAML_SCALAR_EVENT:
                if (key == NULL) {
                    key = strdup((char*)event.data.scalar.value);
                    break;
                } else {
                    if ( strcmp(key, "id") == 0 ) {
                        process->Id = strdup((char*)event.data.scalar.value);
                        // printf("key: %s, val: %s\n", key, process->Id);
                    } else if ( strcmp(key, "name") == 0 ) {
                        process->Name = strdup((char*)event.data.scalar.value);
                        // printf("key: %s, val: %s\n", key, process->Name);
                    } else if ( strcmp(key, "pid") == 0 ) {
                        process->Pid = atoi((char*)event.data.scalar.value);
                        // printf("key: %s, val: %d\n", key, process->Pid);
                    } else if ( strcmp(key, "contextDir") == 0 ) {
                        process->ContextDir = strdup((char*)event.data.scalar.value);
                    } else if ( strcmp(key, "image") == 0 ) {
                        break;
                    } else if ( strcmp(key, "job") == 0 ) {
                       break; 
                    } else if ( strcmp(key, "env") == 0 ) {
                        break;
                    } else if ( strcmp(key, "network") == 0 ) {
                        break;
                    }
                }
                free(key);
                key = NULL;
                break;
            
            case YAML_MAPPING_START_EVENT:
                if (key == NULL) {
                    break;
                } else if (strcmp(key, "image") == 0) {
                    free(key);
                    key = NULL;
                    struct Image* image = (struct Image*)calloc(1, sizeof(struct Image));
                    parse_image(&parser, image);
                    process->Image = image;
                } else if (strcmp(key, "job") == 0) {
                    free(key);
                    key = NULL;
                    struct ProcessJob* job = (struct ProcessJob*)calloc(1, sizeof(struct ProcessJob));
                    parse_process_job(&parser, job);
                    process->Job = job;
                } else if (strcmp(key, "env") == 0) {
                    free(key);
                    key = NULL;
                    struct ProcessEnv* penv = calloc(1, sizeof(struct ProcessEnv));
                    parse_process_env(&parser, penv);
                    process->Env = penv;
                } else if (strcmp(key, "network") == 0) {
                    free(key);
                    key = NULL;
                    struct ProcessNetwork* net = calloc(1, sizeof(struct ProcessNetwork));
                    parse_process_net(&parser, net);
                    process->Network = net;
                }
                break;

            default:
                break;
        }
        yaml_event_delete(&event);
    }

    if (key != NULL){
        free(key);
        key = NULL;
    }

    printf("parsed the yaml\n");
    yaml_parser_delete(&parser);
    fclose(file);
    print_parsed_process(process);
    // free_process(process);
}


void parse_image(yaml_parser_t* parser, struct Image* image) {
    yaml_event_t event;
    char* key = NULL;

    while(1) {
        if (!yaml_parser_parse(parser, &event)) {
            fprintf(stderr, "parser error: %d\n", parser->error);
            break;
        } 

        switch(event.type) {
            case YAML_MAPPING_START_EVENT:
                if (key != NULL){
                    free(key);
                }
                key = NULL;
                break;

            case YAML_SCALAR_EVENT:
                if ( key == NULL ) {
                    key = strdup((char*)event.data.scalar.value);
                } else {
                    if ( strcmp(key, "id") == 0 ) {
                        image->Id = strdup((char*)event.data.scalar.value);
                        // printf("key: %s, val: %s\n", key, image->Id);
                    } else if ( strcmp(key, "name") == 0 ) {
                        image->Name = strdup((char*)event.data.scalar.value);
                        // printf("key: %s, val: %s\n", key, image->Name);
                    } else if ( strcmp(key, "context_temp_dir") == 0 ) {
                        image->ContextTempDir = strdup((char*)event.data.scalar.value);
                        // printf("key: %s, val: %s\n", key, image->ContextTempDir);
                    } else if ( strcmp(key, "imgpath") == 0 ) {
                        image->ImgPath = strdup((char*)event.data.scalar.value);
                        // printf("key: %s, val: %s\n", key, image->ImgPath);
                    } else if ( strcmp(key, "tag") == 0 ) {
                        image->Tag = strdup((char*)event.data.scalar.value);
                        // printf("key: %s, val: %s\n", key, image->Tag);
                    } else if ( strcmp(key, "created") == 0 ) {
                        image->Created = strdup((char*)event.data.scalar.value);
                        // printf("key: %s, val: %s\n", key, image->Created);
                    }
                    free(key);
                    key = NULL;
                }
                yaml_event_delete(&event);
                break;

            case YAML_MAPPING_END_EVENT:
                if ( key != NULL ){
                    free(key);
                    key = NULL;
                }
                yaml_event_delete(&event);
                printf("mapping end event\n");
                return; 

            default:
                break;
        }
    }
    if (key != NULL){
        free(key);
        key = NULL;
    }

}

void parse_process_job(yaml_parser_t* parser, struct ProcessJob* job) {
    yaml_event_t event;
    char* key = NULL;

    while(1) {
        if (!yaml_parser_parse(parser, &event)) {
            fprintf(stderr, "parser error: %d\n", parser->error);
            break;
        } 

        switch(event.type){
            case YAML_MAPPING_START_EVENT:
                if( key != NULL ){
                    free(key);
                }
                key = NULL;
                yaml_event_delete(&event);
                break;

            case YAML_SCALAR_EVENT:
                if ( key == NULL ) {
                    key = strdup((char*)event.data.scalar.value);
                    yaml_event_delete(&event);
                } else {
                    if ( strcmp(key, "name") == 0 ) {
                        job->Name = strdup((char*)event.data.scalar.value);
                        // printf("key: %s, val: %s\n", key, image->Name);
                    } else if ( strcmp(key, "command") == 0 ) {
                        yaml_event_delete(&event);
                        break;
                    }
                    free(key);
                    key = NULL;
                }
                yaml_event_delete(&event);
                break;

            case YAML_SEQUENCE_START_EVENT:
                if ( key == NULL ){
                    break;
                }
                if ( strcmp(key, "command") == 0 ){
                    job->Command = (struct ProcessJobCommand*)calloc(1, sizeof(struct ProcessJobCommand));
                    parse_job_command(parser, job->Command);
                }
                break;

            case YAML_MAPPING_END_EVENT:
                if ( key != NULL ){
                    free(key);
                    key = NULL;
                }
                yaml_event_delete(&event);
                printf("mapping end event\n");
                return; 

            default:
                yaml_event_delete(&event);
        }
    }
    if (key != NULL){
        free(key);
        key = NULL;
    }
}

void parse_job_command(yaml_parser_t* parser, struct ProcessJobCommand* job) { 
    yaml_event_t event;

    job->argc = 0;
    int argc = 0;
    char* args[MAX_JOB_CMD_ARGS];
    memset(args, 0, sizeof(args));


    while(1) {
        if (!yaml_parser_parse(parser, &event)) {
            fprintf(stderr, "parser error: %d\n", parser->error);
            break;
        } 

        switch(event.type) {
            case YAML_SCALAR_EVENT:
                if (argc == MAX_JOB_CMD_ARGS ){
                    perror("too many args in cmd");
                    exit(1);
                }
                args[argc] = strdup((char*)event.data.scalar.value); 
                argc = argc + 1;
                if ( args == NULL ) {
                    perror("error realloc");
                    exit(1);
                }
                yaml_event_delete(&event);
                break;

            case YAML_SEQUENCE_END_EVENT:
                printf("sequence delete\n");
                job->argc = argc;
                char** argsptr = (char**)calloc(argc+1, sizeof(char*));
                if(argc > 0){
                    for(int c=0; c < argc; c++){
                        argsptr[c] = args[c];
                    }
                    argsptr[argc] = NULL;
                    job->args = argsptr;
                    job->command = strdup(args[0]);
                }
                yaml_event_delete(&event);
                return; 
            default:
                yaml_event_delete(&event);
        }
    }
}


void parse_process_env(yaml_parser_t* parser, struct ProcessEnv* penv) {
    yaml_event_t event;

    int env_count = 0; 
    struct Env* env_vars[MAX_PROC_ENV];
    struct Env* curr_env = NULL;
    memset(env_vars, 0, sizeof(env_vars));

    char* key = NULL;
    char* val = NULL;
    
    while(1) {
        if (!yaml_parser_parse(parser, &event)) {
            fprintf(stderr, "parser error: %d\n", parser->error);
            break;
        } 

        switch (event.type)
        {
            case YAML_SCALAR_EVENT:
                if (env_count == MAX_PROC_ENV ){
                    perror("too many envs");
                    exit(1);
                }
                if ( key == NULL ){
                    key = strdup((char*)event.data.scalar.value); 
                } else {
                    if ( key != NULL ){
                        val = strdup((char*)event.data.scalar.value); 
                        curr_env = (struct Env*)calloc(1, sizeof(struct Env));
                        curr_env->Key = key;
                        curr_env->Val = val;
                        key = NULL;
                        val = NULL;
                        env_vars[env_count] = curr_env;
                        curr_env = NULL;
                        env_count = env_count + 1;
                    } else {
                        free(key);
                        yaml_event_delete(&event);
                        perror("invalid yaml");
                        exit(1);
                    }
                }
                break;

            case YAML_MAPPING_END_EVENT:
                struct Env** env = (struct Env**)calloc(env_count, sizeof(struct Env*));
                if (env_count > 0){
                    for(int e=0; e < env_count; e++){
                        env[e] = env_vars[e];
                    }
                }
                penv->env = env;
                penv->count = env_count;
                yaml_event_delete(&event);
                return; 
                        
            default:
                break;
        }

        yaml_event_delete(&event);
    }
}


void parse_process_net(yaml_parser_t* parser, struct ProcessNetwork* net) {
    yaml_event_t event;

    char* key = NULL;

    while(1) {
        if (!yaml_parser_parse(parser, &event)) {
            fprintf(stderr, "parser error: %d\n", parser->error);
            break;
        } 

        switch (event.type)
        {
            case YAML_SCALAR_EVENT:
                if(key == NULL){
                    key = strdup((char*)event.data.scalar.value);
                } 
                break;

            case YAML_SEQUENCE_START_EVENT:
                if( key == NULL ){
                    break;
                } else {
                    if ( strcmp(key, "ports") == 0 ){
                        net->pm = (struct PortMapping*)calloc(1, sizeof(struct PortMapping));
                        parse_pnet_ports(parser, net); 
                    }
                    free(key);
                    key = NULL;
                }
                break;

            case YAML_MAPPING_END_EVENT:
                yaml_event_delete(&event);
                return;
            
            default:
                break;
        }

        yaml_event_delete(&event);
    }

}

void parse_pnet_ports(yaml_parser_t* parser, struct ProcessNetwork* net) {
    yaml_event_t event;

    int nports = 0;
    struct PortMap* pmap[MAX_PORT_MAPS];
    memset(pmap, 0, sizeof(pmap));

    struct PortMapping* port_mapping = net->pm;

    while(1) {
        if (!yaml_parser_parse(parser, &event)) {
            fprintf(stderr, "parser error: %d\n", parser->error);
            break;
        }

        switch (event.type)
        {
            case YAML_MAPPING_START_EVENT:
                /* parse_pnet_port_map */
                struct PortMap *portmap = calloc(1, sizeof(struct PortMap));
                parse_pnet_port_map(parser, portmap); 
                nports = nports + 1;
                pmap[nports-1] = portmap;
                break;


            case YAML_SEQUENCE_END_EVENT:
                /* code */
                port_mapping->nports = nports;
                port_mapping->pmap = (struct PortMap**)calloc(nports, sizeof(struct PortMap*));
                for(int p=0; p< nports; p++){
                    port_mapping->pmap[p] = pmap[p];
                }
                yaml_event_delete(&event);
                return;
            
            default:
                break;
        }

        yaml_event_delete(&event);

    }
}


void parse_pnet_port_map(yaml_parser_t* parser, struct PortMap* pm){
    yaml_event_t event;

    char* key = NULL;


    while(1) {
        if (!yaml_parser_parse(parser, &event)) {
            fprintf(stderr, "parser error: %d\n", parser->error);
            break;
        }

        switch (event.type)
        {
            case YAML_SCALAR_EVENT:
                /* code */
                if(key == NULL) {
                    key = strdup((char*)event.data.scalar.value); 
                } else {
                    if( strcmp(key, "hostPort") == 0 ) {
                        pm->HostPort = strdup((char*)event.data.scalar.value); 
                    } else if ( strcmp(key, "procPort") == 0 ) {
                        pm->ProcPort = strdup((char*)event.data.scalar.value); 
                    }
                    free(key);
                    key = NULL;
                }
                break;

            case YAML_MAPPING_END_EVENT:
                /* code */
                if ( key != NULL ) {
                    free(key);
                    key = NULL;
                }
                yaml_event_delete(&event);
                return;
            
            default:
                break;
        }

        yaml_event_delete(&event);
    }
}