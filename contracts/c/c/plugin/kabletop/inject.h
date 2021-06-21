
#ifndef CKB_LUA_KABLETOP_INJECT
#define CKB_LUA_KABLETOP_INJECT

#include "../inject.h"
#include "core.h"

int inject_kabletop_functions(lua_State *L, int herr)
{
    inject_ckb_functions(L);

    luaL_dostring(L, "                  \
        _winner = 0                     \
        function _set_random_seed(x, y) \
            math.randomseed(x, y)       \
        end                             \
    ");

    extern const char *_GAME_CHUNK;
    extern const unsigned int _GAME_CHUNK_SIZE;
    if (luaL_loadbuffer(L, _GAME_CHUNK, _GAME_CHUNK_SIZE, "kabletop-context-init")
        || lua_pcall(L, 0, 0, herr))
    {
        ckb_debug("Invalid lua script: please check context code.");
        return KABLETOP_WRONG_LUA_CONTEXT_CODE;
    }

    return CKB_SUCCESS;
}

#endif