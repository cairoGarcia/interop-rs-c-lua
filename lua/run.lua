-- local lib
local ll = require("lib_c")

print(ll.get_i32("Input i32: "))
print(ll.get_string("Input string: "))
print(ll.get_char("Input char: "))
ll.square(ll.get_u8("Square size (u8):  "))
