#include <time.h>
#include "plugin/plugin.h"

int ckb_load_script_hash(void* addr, uint64_t* len, size_t offset);
int ckb_debug(const char* s);

int contract_error_handler(lua_State *L)
{
    const char *error = lua_tostring(L, -1);
    ckb_debug(error);
    return 0;
}

int main()
{
    // Init lua global time seed with script hash for CKB contract only
    uint64_t len = 32;
    unsigned char lock_hash[32];
    ckb_load_script_hash(lock_hash, &len, 0);

    time_t ckb_time = 0;
    for (int i = 0; i < sizeof(time_t); ++i)
    {
        ckb_time = (ckb_time << 8) | (lock_hash[i] >> 1);
    }
    clock_t ckb_clock = 0;
    for (int i = sizeof(time_t); i < sizeof(time_t) + sizeof(clock_t); ++i)
    {
        ckb_clock = (ckb_clock << 8) | (lock_hash[i] >> 1);
    }

    // Init lua status (or context)
    lua_State *L = luaL_newstate(ckb_time, ckb_clock);

    // Load error handler for contract error print
    lua_pushcfunction(L, contract_error_handler);
    int herr = lua_gettop(L);

	int ret = 0;
	CHECK_RET(plugin_init(L, herr));
	CHECK_RET(plugin_verify(L, herr));

    return ret;
}
