#include <stdio.h>
#include "lib.h"

int main()
{
    printf("%s\n", get_string("String: "));
    printf("%d\n", get_i32("i32: "));
    printf("%d\n", get_u8("u8: "));
    printf("%c\n", get_char("char: "));
    square(get_i32("square size: "));
    return 0;
}
