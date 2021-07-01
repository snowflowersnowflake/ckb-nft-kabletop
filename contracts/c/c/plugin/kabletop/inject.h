#ifndef CKB_LUA_KABLETOP_INJECT
#define CKB_LUA_KABLETOP_INJECT

#include "../inject.h"
#include "core.h"
#include "luacode.c"

int inject_kabletop_functions(lua_State *L, int herr)
{
    inject_ckb_functions(L);

	// load internal code
    luaL_dostring(L, "                  \
        _winner = 0                     \
        function _set_random_seed(x, y) \
            math.randomseed(x, y)       \
        end                             \
    ");

	// load native code
    if (luaL_loadbuffer(L, (const char *)_GAME_CHUNK, _GAME_CHUNK_SIZE, "native")
        || lua_pcall(L, 0, 0, herr))
    {
        ckb_debug("Invalid lua script: please check native code.");
        return KABLETOP_WRONG_LUA_CONTEXT_CODE;
    }

	// load celldep code
	const uint8_t prefix[] = "kabletop:";
	const uint64_t psize = sizeof(prefix) - 1;
	for (size_t i = 0; 1; ++i)
	{
		uint8_t check[psize];
		uint64_t size = psize;
		int ret = ckb_load_cell_data(check, &size, 0, i, CKB_SOURCE_CELL_DEP);
        if (ret == CKB_INDEX_OUT_OF_BOUND)
        {
            break;
        }
		if (ret != CKB_SUCCESS)
		{
			return KABLETOP_WRONG_LUA_CONTEXT_CODE;
		}
		if (size <= psize || memcmp(check, prefix, psize))
		{
			continue;
		}

		uint8_t *luacode = (uint8_t *)malloc(size - psize);
		ckb_load_cell_data(luacode, &size, psize, i, CKB_SOURCE_CELL_DEP);

		if (luaL_loadbuffer(L, (const char *)luacode, size, "celldep")
			|| lua_pcall(L, 0, 0, herr))
		{
			free(luacode);
			ckb_debug("Invalid lua script: please check celldep code.");
			return KABLETOP_WRONG_LUA_CONTEXT_CODE;
		}
		free(luacode);
	}

    return CKB_SUCCESS;
}

#endif