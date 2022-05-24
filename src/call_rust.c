#include <stdio.h>
#include <stdlib.h>
#include "crs.h"

int main(void) {
    // hello_from_rust();
    char *str = concat_str("hello ", "world");
    char *empty = concat_str("hello ", NULL);
    printf("concat_strs: '%s'\n", str);
    printf("string + Null concat: '%s'\n", empty);

    char intstr[] = "-1234,+123-456";

    int i = int_from_str(intstr);
    printf("int_from_str: %d\n", i);

    int *is = ints_from_str(intstr);
    for (int i = 0; is[i]; ++i) {
        printf("i[%d] = %d\n", i, is[i]);
    }

    printf("cmp result: (%s == hello world) %d\n", str, cmp_str(str, "hello world"));

    char **sepstr = sep_str("hello,world", ",");

    for (int i = 0; sepstr[i]; ++i) {
        printf("%s\n", sepstr[i]);
        free(sepstr[i]);
    }

    free(str);
    free(sepstr);
    free(empty);
    free_int_arr(is);
    return 0;
}