#include "logger.h"

#include<stdio.h>
#include<string.h>
#include<unistd.h>


void log_info(struct LogContext* ctx, char* msg) {
    if ( ctx == NULL || msg == NULL ) {
        return;
    }

    int fd = ctx->outfd;
    char* tag = "[info] ";
    write(fd, tag, strlen(tag));
    write(fd, msg, strlen(msg));
}

void log_debug(struct LogContext* ctx, char* msg) {
    if ( ctx == NULL || msg == NULL ) {
        return;
    }

    int fd = ctx->outfd;
    char* tag = "[debug] ";
    write(fd, tag, strlen(tag));
    write(fd, msg, strlen(msg));
}


void log_warn(struct LogContext* ctx, char* msg) {
    if ( ctx == NULL || msg == NULL ) {
        return;
    }

    int fd = ctx->errfd;
    char* tag = "[warning] ";
    write(fd, tag, strlen(tag));
    write(fd, msg, strlen(msg));
}

void log_error(struct LogContext* ctx, char* msg) {
    if ( ctx == NULL || msg == NULL ) {
        return;
    }

    int fd = ctx->errfd;
    char* tag = "[error] ";
    write(fd, tag, strlen(tag));
    write(fd, msg, strlen(msg));
}


void get_std_logger(struct LogContext* ctx) {
    ctx->outfd = fileno(stdout);
    ctx->errfd = fileno(stderr);
}