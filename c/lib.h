// #include <lua.h>
// #include <lauxlib.h>
// #include <lualib.h>

// rust safe function
char *get_string(char *s);
char get_char(char *s);
int get_i32(char *s);
unsigned char get_u8(char *s);

// rust safe function
void square(unsigned char size);

// static int echo (lua_State *L) {
//     square(get_u8("square size: "));
// }
// static const struct luaL_Reg lib_c [] = {
//     {"echo", echo},
//     {NULL, NULL},  /* sentinel */
// };
