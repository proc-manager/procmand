#ifndef PROCESS_H
#define PROCESS_H

#define MAX_JOB_CMD_ARGS 20
#define MAX_PROC_ENV 50
#define MAX_PORT_MAPS 30
#define STACKSIZE (1024*1024)


// structs

struct Env {
    char* Key;
    char* Val;
};

struct ProcessEnv {
    int count;
    struct Env** env;
};

struct Image {
    char* Id;
    char* Name;
    char* ContextTempDir;
    char* ImgPath;
    char* Tag;
    char* Created;
};

struct PortMap {
    char* HostPort;
    char* ProcPort;
};

struct PortMapping {
    int nports;
    struct PortMap** pmap;
};

struct ProcessNetworkNamespace {
    char* NamespaceId; 
};

struct ProcessNetwork {
    struct PortMapping* pm;
};

struct ProcessJobCommand{
    char* command; 
    char** args; 
    int argc;
};

struct ProcessJob {
    char* Name;
    struct ProcessJobCommand* Command;
};



struct Process {
    // id of the process 
    char* Id;
    char* Name;
    int Pid;
    char* ContextDir;
    struct Image* Image;
    struct ProcessJob* Job;
    struct ProcessEnv* Env;
    struct ProcessNetwork* Network;

    // params from outside the yaml 
    int fd[2];
    char* Stack; // the allocated stack - must be freed
    char* Rootfs; // pointer to rootfs 
    int ExitStatus; // exit status
};

// functions 

/*
    Free function for dynamically allocated struct Process. 
*/
void free_process(struct Process* process);

/*
    Free function for dynamically allocated struct Env
*/
void free_env(struct Env* env);

/*
    Free function for dynamically allocated struct ProcessEnv
*/
void free_process_env(struct ProcessEnv* penv);

/*
   Free function for dynamically allocated struct Image 
*/
void free_image(struct Image* image);

/*
    Free function for dynamically allocated struct PortMap
*/
void free_port_map(struct PortMap* p); 

/*
    Free function for dynamically allocated struct PortMapping. 
*/
void free_network_port_mapping(struct PortMapping* pm);

/*
    Free function for dynamically allocated ProcessNetworkNamespace
*/
void free_process_network_namespace(struct ProcessNetworkNamespace *pns);

/*
    Free function for dynamically allocated struct ProcessNetwork
*/
void free_process_network(struct ProcessNetwork* net);

/*
    Free function for dynamically allocated struct ProcessJobCommand
*/
void free_process_job_command(struct ProcessJobCommand *cmd);

/*
    Free function for dynamically allocated struct ProcessJob
*/
void free_process_job(struct ProcessJob* job);

/*
    Free function for the stack allocated for the process
*/
void free_process_stack(struct Process* proc);

/*
    Free function for the rootfs of the process
*/
void free_process_rootfs(struct Process* proc);



#endif