#include <stdio.h>
#include <stdlib.h>
#include <sys/syscall.h>
#include <unistd.h>
#define SYS_hello 548

int main(void) {
    char *buf = (char *)malloc(40 * sizeof(char));
    printf("test enough buffer: ");
    if (syscall(SYS_hello, buf, 39) == -1) {
        printf("not enough buffer\n");
    } else {
        printf("enough buffer, %s", buf);
    }
    printf("\ntest not enough buffer: ");
    if (syscall(SYS_hello, buf, 0) == -1) {
        printf("not enough buffer\n");
    } else {
        printf("enough buffer, %s", buf);
    }
    while (1) {
    }
}
