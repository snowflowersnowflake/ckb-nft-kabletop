#ifndef CKB_LUA_PLUGIN
#define CKB_LUA_PLUGIN

#include "lauxlib.h"

int plugin_init(lua_State *L, int herr);

int plugin_verify(lua_State *L, int herr);

#endif
