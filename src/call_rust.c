#include <stdio.h>
#include <stdlib.h>
#include "crs.h"

int main(void) {
    // hello_from_rust();
    char *str = concat_str("hello ", "world");
    if (str != NULL) {
        printf("concat_strs: '%s'\n", str);
    }

    char intstr[] = "-1234,+123-456+123123123-12,13";
    char *cp = new_str(intstr);
    printf("strdup: %s\n", cp);
    printf("diff: %d\n", cmp_str(intstr, cp));
    char *emp = new_str_empty();
    printf("empty: %s\n", emp);

    int i = int_from_str(intstr);
    printf("int_from_str: %d\n", i);

    slice_boxed_int32_t ints = ints_from_str(intstr);
    for (size_t i = 0; i < ints.len; i++) {
        printf("%d\n", ints.ptr[i]);
    }

    char sepstr[] = "hello,world,!";
    slice_boxed_char_ptr_t slice = split_str(sepstr, ", ");
    if (slice.len != 1) {
        for (size_t i = 0; i < slice.len; i++) {
            printf("'%s'\n", slice.ptr[i]);
        }
    }

    // char sepstr2[] = "hello,world !";
    // slice_boxed_char_ptr_t slice2 = sep_str(sepstr2, "");
    // if (slice2.ptr == NULL) return 1;

    // for (size_t i = 0; i < slice2.len; i++) {
    //     printf("%s\n", slice2.ptr[i]);
    // }

    printf("cmp result: (%s == hello world) %d\n", str, cmp_str(str, "hello world"));

    free_str(str);
    free_str(cp);
    free_str_arr(slice);
    free_int_arr(ints);
    return 0;
}