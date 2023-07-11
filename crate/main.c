#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>

#include <sched.h>
#include <unistd.h>
#include <errno.h>

#include <sys/types.h>
#include <sys/wait.h>
#include <sys/mount.h>

#define STACK_SIZE (1024 * 1024)

static char child_stack[STACK_SIZE];

int mount_fs(void) {
	const char *source = "proc";
    const char *target = "/proc";
    const char *filesystem = "proc";
    unsigned long mountflags = 0;
    const void *data = NULL;

    int mount_result = mount(source, target, filesystem, mountflags, data);
    if (mount_result == 0) {
        printf("Filesystem mounted successfully.\n");
    } else {
        perror("Mount failed");
		return EXIT_FAILURE;
    }

	return EXIT_SUCCESS;
}

int drop_root_privileges(void) {
	uid_t uid = getuid();

	if ((uid = getuid()) == 0) {
		const char *sudo_uid = secure_getenv("SUDO_UID");
		uid = (uid_t) strtoll(sudo_uid, NULL, 10);
	}

	if (setuid(uid) != 0) {
		perror("setgid");
		return EXIT_FAILURE;
	}

	if (seteuid(0) == 0) {
		fprintf(stderr, "Could not drop root privileges!\n");
		return EXIT_FAILURE;
	}

	return EXIT_SUCCESS;
}

void setup_container(void) {
	if(mount_fs() != EXIT_SUCCESS) {
		exit(-1);
	}

	if(drop_root_privileges() != EXIT_SUCCESS) {
		exit(-1);
	}

   if (unshare(CLONE_NEWUSER) == -1) {
        perror("unshare");
	}
}

int child_process(void *arg) {
	setup_container();
	
    char **cmd = (char **)arg;

    if (execvp(cmd[0], cmd) == -1) {
        perror("execvp");
        exit(EXIT_FAILURE);
    }

    return 0;
}

int main(int argc, char *argv[]) {
    if (argc < 2) {
        fprintf(stderr, "Usage: %s [program] [args...]\n", argv[0]);
        exit(EXIT_FAILURE);
    }

	const long flags = CLONE_NEWIPC | CLONE_NEWNET | CLONE_NEWNS | 
					   CLONE_NEWPID | CLONE_NEWUTS |
					   SIGCHLD;

    pid_t pid = clone(child_process, child_stack + STACK_SIZE, flags, argv + 1);
    if (pid == -1) {
        perror("clone");
        exit(EXIT_FAILURE);
    }

    waitpid(pid, NULL, 0);

    return 0;
}
