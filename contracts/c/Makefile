TARGET := riscv64-unknown-elf
CC := $(TARGET)-gcc
LD := $(TARGET)-gcc
STRIP := $(TARGET)-strip
# CFLAGS := -Os -DCKB_NO_MMU -D__riscv_soft_float -D__riscv_float_abi_soft -DCKB_C_STDLIB_PRINTF -fvisibility=hidden
CFLAGS := -fPIC -Os -fno-builtin-memcmp -fno-builtin-printf -nostdlib -nostartfiles -fvisibility=hidden
SECP256k1 := deps/ckb-lib-secp256k1
STDLIB := deps/ckb-c-stdlib
APP_CFLAGS := $(CFLAGS) -Ilua -Ic -I$(STDLIB) -I$(STDLIB)/molecule -I$(SECP256k1) -I$(SECP256k1)/secp256k1 -I$(SECP256k1)/secp256k1/src -Wall -Werror -Wno-unused-function -Wno-nonnull-compare -Wno-unused-value
LDFLAGS := -lm -Wl,-static -fdata-sections -ffunction-sections -Wl,--gc-sections

via-docker: clean-kabletop build/kabletop
	cp ./build/kabletop $(ARGS)

all: build/luavm build/kabletop

build/luavm: build/entry.o build/luavm.o build/liblua.a
	$(LD) $^ -o $@ $(LDFLAGS)
	$(STRIP) $@

build/kabletop: build/entry.o build/kabletop.o build/liblua.a
	$(LD) $^ -o $@ $(LDFLAGS)
	$(STRIP) $@

build/entry.o: c/entry.c
	mkdir -p build
	$(CC) $(APP_CFLAGS) $< -c -o $@

build/luavm.o: c/plugin/luavm/plugin.c
	$(CC) $(APP_CFLAGS) $< -c -o $@

build/kabletop.o: c/plugin/kabletop/plugin.c secp256k1
	$(CC) $(APP_CFLAGS) $< -c -o $@

secp256k1:
	cd deps/ckb-lib-secp256k1/secp256k1 && \
		./autogen.sh && \
		CC=$(CC) LD=$(LD) ./configure --with-bignum=no --enable-ecmult_static_precomputation=no --enable-endomorphism --enable-module-recovery --host=$(TARGET)

build/liblua.a:
	KABLETOP=1 make -C ./lua a
	cp ./lua/build/liblua.a $@

clean-kabletop:
	rm -rf build/*.o build/kabletop

clean:
	rm -rf build/*.o build/*.a build/lua
	make -C ./lua clean
