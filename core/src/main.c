#include <unistd.h>
#include <sys/wait.h>
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char* argv[]) {
    int fd[2];
    pipe(fd);
    pid_t pid = fork();

    if ( pid == 0 ) {
        dup2(fd[0], STDIN_FILENO);
    } else {

    }
}