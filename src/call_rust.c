#include <stdio.h>
#include "crs.h"

int main(void) {
    const char *str = concat_str("hello ", "world");
    int i = printf("%s\n", str);
    (void)i;
    return 0;
}