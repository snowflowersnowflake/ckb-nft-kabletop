TARGET := riscv64-unknown-elf
CC := $(TARGET)-gcc
LD := $(TARGET)-gcc
CFLAGS := -Os -DCKB_NO_MMU -D__riscv_soft_float -D__riscv_float_abi_soft -DCKB_C_STDLIB_PRINTF -g
SECP256k1 := deps/ckb-lib-secp256k1
APP_CFLAGS := $(CFLAGS) -Ilua -Ic -Ideps/ckb-c-stdlib -Ideps/molecule -I$(SECP256k1) -I$(SECP256k1)/secp256k1 -I$(SECP256k1)/secp256k1/src -Wall -Werror -Wno-unused-function -Wno-nonnull-compare -Wno-unused-value
LDFLAGS := -lm -Wl,-static -fdata-sections -ffunction-sections -Wl,--gc-sections -Wl,-g

via-docker: clean-kabletop build/kabletop
	cp ./build/kabletop ./build/debug/

all: build/luavm build/kabletop

build/luavm: build/entry.o build/luavm.o build/liblua.a
	$(LD) $^ -o $@ $(LDFLAGS)

build/kabletop: build/entry.o build/kabletop.o build/liblua.a #build/luacode.o
	$(LD) $^ -o $@ $(LDFLAGS)

build/entry.o: c/entry.c
	$(CC) $(APP_CFLAGS) $< -c -o $@

build/luavm.o: c/plugin/luavm/plugin.c
	$(CC) $(APP_CFLAGS) $< -c -o $@

build/kabletop.o: c/plugin/kabletop/plugin.c
	$(CC) $(APP_CFLAGS) $< -c -o $@

# build/luacode.o: c/plugin/kabletop/luacode.c
# 	$(CC) $(APP_CFLAGS) $< -c -o $@

build/liblua.a:
	make -C ./lua a
	cp ./lua/liblua.a $@

clean-kabletop:
	rm -rf build/*.o build/kabletop

clean:
	rm -rf build/*.o build/*.a build/lua
	make -C ./lua clean
