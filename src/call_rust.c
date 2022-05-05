#include <stdio.h>
#include "crs.h"

int main(void) {
    const char *str = concat_strs("hello ", "world");
    printf("concat_strs: %s\n", str);

    int i = int_from_str("123");
    printf("int_from_str: %d\n", i);

    return 0;
}