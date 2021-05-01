
#ifndef CKB_LUA_KABLETOP_INJECT
#define CKB_LUA_KABLETOP_INJECT

#include "../inject.h"

void inject_kabletop_functions(lua_State *L)
{
    inject_ckb_functions(L);

    luaL_dostring(L, "                  \
        _winner = 0                     \
        function _set_random_seed(x, y) \
            math.randomseed(x, y)       \
        end                             \
    ");
}

#endif