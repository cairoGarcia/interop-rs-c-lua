#include <lua.h>
#include <lauxlib.h>
#include <lualib.h>

// rust safe function
// char *get_string(char *s);
// char get_char(char *s);
// int get_i32(char *s);
// unsigned char get_u8(char *s);

// rust safe function
void square(unsigned char size);

static int quadrado (lua_State *L) {
    double a = luaL_checknumber(L, 1);
    unsigned char b = a;
    square(b);
    return 1;
}

static const luaL_Reg lib_c [] = {
    {"quadrado", quadrado},
    {NULL, NULL},  /* sentinel */
};

int luaopen_lib_c(lua_State *L) {
    luaL_newlib(L, lib_c);
    return 1;
}
