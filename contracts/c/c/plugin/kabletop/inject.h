
#ifndef CKB_LUA_KABLETOP_INJECT
#define CKB_LUA_KABLETOP_INJECT

#include "../inject.h"

void inject_kabletop_functions(lua_State *L)
{
    inject_ckb_functions(L);
}

#endif