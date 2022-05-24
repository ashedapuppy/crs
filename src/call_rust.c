#include <stdio.h>
#include <stdlib.h>
#include "crs.h"

int main(void) {
    // hello_from_rust();
    char *str = concat_str("hello ", "world");
    char *empty = concat_str("hello ", NULL);
    printf("concat_strs: '%s'\n", str);
    printf("string + Null concat: '%s'\n", empty);

    char intstr[] = "-1234,+123-456+123123123-12,13";

    int i = int_from_str(intstr);
    printf("int_from_str: %d\n", i);

    IntArray is = ints_from_str(intstr);
    for (size_t i = 0; i < is.len; ++i) {
        printf("i[%lu] = %d\n", i, is.data[i]);
    }

    printf("cmp result: (%s == hello world) %d\n", str, cmp_str(str, "hello world"));

    char **sepstr = sep_str("hello,world,a,b,c,123", ",");

    for (int i = 0; sepstr[i]; i++) {
        printf("'%s'\n", sepstr[i]);
    }

    // freeing all memory locations allocated by Rust
    for (int i = 0; sepstr[i]; ++i) {
        free(sepstr[i]);
    }
    free(sepstr);
    free_int_array(is);
    free(str);
    free(empty);
    return 0;
}