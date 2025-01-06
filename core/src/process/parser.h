#ifndef PROCESS_PARSER_H
#define PROCESS_PARSER_H

#include <yaml.h>
#include "process.h"

void parse_process_yaml(char* filepath, struct Process* process);
void parse_image(yaml_parser_t* parser, struct Image* image);
void parse_process_job(yaml_parser_t* parser, struct ProcessJob* job);
void parse_job_command(yaml_parser_t* parser, struct ProcessJobCommand* job);
void parse_process_env(yaml_parser_t* parser, struct ProcessEnv* penv);
void parse_process_net(yaml_parser_t* parser, struct ProcessNetwork* net);
void parse_pnet_ports(yaml_parser_t* parser, struct ProcessNetwork* net);
void parse_pnet_port_map(yaml_parser_t* parser, struct PortMap* pm);
#endif // PARSE_PROC_SPEC_H