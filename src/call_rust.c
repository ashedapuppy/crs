#include <stdio.h>
#include <stdlib.h>
#include "crs.h"

int main(void) {
    // hello_from_rust();
    char *str = concat_str("hello ", "world");
    printf("concat_strs: '%s'\n", str);

<<<<<<< HEAD
    char intstr[] = "-1234,+123-456+123123123";
=======
    char intstr[] = "-1234,+123-456+123123123-12,13";
    printf("strdup: %s\n", dup_str(intstr));
>>>>>>> safer_ffi

    int i = int_from_str(intstr);
    printf("int_from_str: %d\n", i);

    slice_boxed_int32_t ints = ints_from_str(intstr);
    for (size_t i = 0; i < ints.len; i++) {
        printf("%d\n", ints.ptr[i]);
    }

<<<<<<< HEAD
    printf("cmp result: (%s == hello world) %d\n", str, eq_str(str, "hello world"));

    char **sepstr = sep_str("hello,world", ",");
=======
    char sepstr[] = "hello,world !";
    slice_boxed_char_ptr_t slice = sep_str(sepstr, ", ");
>>>>>>> safer_ffi

    for (size_t i = 0; i < slice.len; i++) {
        printf("%s\n", slice.ptr[i]);
    }

    printf("cmp result: (%s == hello world) %d\n", str, cmp_str(str, "hello world"));

    rust_free_string(str);
    rust_free_string_array(slice);
    rust_free_int_array(ints);
    return 0;
}