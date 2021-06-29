TARGET := riscv64-unknown-elf
CC := $(TARGET)-gcc
LD := $(TARGET)-gcc
STRIP := $(TARGET)-strip
CFLAGS := -fPIC -Os -fno-builtin-memcmp -fno-builtin-printf -nostdlib -nostartfiles -fvisibility=hidden
SECP256k1 := deps/ckb-lib-secp256k1
STDLIB := deps/ckb-c-stdlib
APP_CFLAGS := $(CFLAGS) -Ic -I$(STDLIB) -I$(STDLIB)/molecule -I$(SECP256k1) -I$(SECP256k1)/secp256k1 -I$(SECP256k1)/secp256k1/src -Wall -Werror -Wno-unused-function -Wno-nonnull-compare -Wno-unused-value
LDFLAGS := -lm -Wl,-static -fdata-sections -ffunction-sections -Wl,--gc-sections

via-docker: clean-kabletop build/kabletop
	cp ./build/kabletop $(ARGS)

all: build/kabletop

build/kabletop: build/kabletop.o
	$(LD) $^ -o $@ $(LDFLAGS)
	$(STRIP) $@

build/kabletop.o: c/entry.c
	$(CC) $(APP_CFLAGS) $< -c -o $@

clean-kabletop:
	rm -rf build/*.o build/kabletop

clean:
	rm -rf build/*.o build/*.a
