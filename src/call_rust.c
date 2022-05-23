#include <stdio.h>
#include "crs.h"



int main(void) {
    char *str = concat_str("hello ", "world");
    printf("concat_strs: %s\n", str);

    int i = int_from_str("1234");
    printf("int_from_str: %d\n", i);

    printf("cmp result: (%s == hello world) %d\n", str, cmp_str(str, "hello world"));

    free_str(str);
    return 0;
}