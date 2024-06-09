#include <stdio.h>
#include <unistd.h>

int main() {
    FILE *file;
    file = fopen("output.txt", "a");
    if (file == NULL) {
        perror("Unable to open file");
        return 1;
    }

    while (1) {
        fprintf(file, "hello\n");
        fflush(file);
        sleep(3);
    }

    fclose(file);
    return 0;
}
