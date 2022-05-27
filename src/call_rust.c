#include <stdio.h>
#include <stdlib.h>
#include "crs.h"

void test_rust_lib(void) {
    // hello_from_rust();
    char *str = concat_str("hello ", "world");
    if (str != NULL) {
        printf("concat_strs: '%s'\n", str);
    }
    printf("cmp result: (%s == hello world) %d\n", str, cmp_str(str, "hello world"));
    free(str);

    char intstr[] = "-1234,+123-456+123123123-12,13";
    char *cp = new_str(intstr);
    printf("strdup: %s\n", cp);
    printf("diff: %d\n", cmp_str(intstr, cp));
    free(cp);

    char *emp = new_str_empty();
    printf("empty: %s\n", emp);

    char *fmtd_str = fmt_int(intstr);
    printf("int_from_str: %s\n", fmtd_str);
    free(fmtd_str);

    char floatstr[] = "-123.123abc";
    char *fmtd_str2 = fmt_double(floatstr);
    printf("double_from_str: %s\n", fmtd_str2);
    free(fmtd_str2);

    slice_boxed_double_t ints = doubles_from_str(intstr);
    char *buf;
    if (ints.len != 1) {
        for (size_t i = 0; i < ints.len; i++) {
            buf = double_to_str(ints.ptr[i]);
            printf("%s\n", buf);
            free(buf);
        }
    }
    free_double_arr(ints);

    char sepstr[] = "hello,world,!";
    slice_boxed_char_ptr_t slice = split_str(sepstr, ", ");
    if (slice.len != 1) {
        for (size_t i = 0; i < slice.len; i++) {
            printf("'%s'\n", slice.ptr[i]);
        }
    }
    free_str_arr(slice);
}

int main(int argc, char **argv) {
    (void)argc; // cast to void to mark as unused
    (void)argv;
    test_rust_lib();
    return 0;
}