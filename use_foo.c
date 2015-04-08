#include <stdio.h>

unsigned int foo();

int main() {
	printf("hello from c\n");
	unsigned int value = foo();
	printf("foo returned %d\n", value);
	return 0;
}