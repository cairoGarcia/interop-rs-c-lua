#include "lib_c.h"
#include <stdio.h>

int main()
{
    char *s = get_string("String: ");
    get_i32("i32: ");
    get_u8("u8: ");
    get_char("char: ");
    square(get_i32("square size: "));
    return 0;
}
