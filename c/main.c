#include "lib_c.h"
#include <stdio.h>

int main()
{
    char *s = get_string();
    printf("%s\n", s);
    square(get_int());
    return 0;
}
