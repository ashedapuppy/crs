#include <stdio.h>
#include "crs.h"

int main(void) {
    hello_from_rust();
    char *str = concat_str("hello ", "world");
    char *empty = concat_str("hello ", NULL);
    printf("concat_strs: '%s'\n", str);
    printf("string + Null concat: '%s'\n", empty);

    int i = int_from_str("1234");
    printf("int_from_str: %d\n", i);

    printf("cmp result: (%s == hello world) %d\n", str, cmp_str(str, "hello world"));

    free_str(str);
    return 0;
}