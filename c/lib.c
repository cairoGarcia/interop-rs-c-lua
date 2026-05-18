#include <lua.h>
#include <lauxlib.h>
#include <lualib.h>

// rust safe function
char *get_string(const char *s);
char get_char(const char *s);
int get_i32(const char *s);
unsigned char get_u8(const char *s);

// rust safe function
void square(unsigned char size);

static int get_stringL(lua_State *L) {
    char *s = get_string(luaL_checkstring(L, 1));
    lua_pushstring(L, s);

    return 1;
};

static int get_charL(lua_State *L) {
    char c = get_char(luaL_checkstring(L, 1));
    lua_pushstring(L, &c);

    return 1;
};

static int get_i32L(lua_State *L) {
    int n = get_i32(luaL_checkstring(L, 1));
    lua_pushnumber(L, n);
    
    return 1;
}

static int get_u8L(lua_State *L) {
    unsigned char n = get_u8(luaL_checkstring(L, 1));
    lua_pushnumber(L, n);
    
    return 1;
}

static int squareL(lua_State *L) {
    double size = luaL_checknumber(L, 1);
    square( (unsigned char) size);
    return 1;
}

static const luaL_Reg lib_c [] = {
    {"square", squareL},
    {"get_u8", get_u8L},
    {"get_i32", get_i32L},
    {"get_string", get_stringL},
    {"get_char", get_charL},
    {NULL, NULL},  /* sentinel */
};

int luaopen_lib_c(lua_State *L) {
    luaL_newlib(L, lib_c);
    return 1;
}
