#include "unistd.h"
#include "stdio.h"
#include "stdlib.h"
#include "assert.h"

void test_open() {
	// O_RDONLY = 0, O_WRONLY = 1
	FILE *fd = fopen("./text.txt", "r");
	assert(fd >= 0);
	char buf[256];
	int size = fread(buf, 256, 1, fd);
	if (size < 0) {
		size = 0;
	}
    puts(buf);
	fclose(fd);
}

int main(void) {
	test_open();
	return 0;
}
