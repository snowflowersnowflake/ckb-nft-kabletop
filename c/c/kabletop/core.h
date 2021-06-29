
#ifndef CKB_LUA_KABLETOP_CORE
#define CKB_LUA_KABLETOP_CORE

#include <stdio.h>
#include "ckb_syscalls.h"
#include "ckb_consts.h"
#include "blockchain.h"
#include "molecule/kabletop.h"

#define MAX_SCRIPT_SIZE 32768
#define MAX_ROUND_SIZE 2048
#define MAX_CHALLENGE_DATA_SIZE 2048
#define MAX_OPERATIONS_PER_ROUND 32
#define MAX_NFT_DATA_SIZE (BLAKE160_SIZE * 256)
#define MAX_ROUND_COUNT 256
#define ERROR_ENCODING -2

#define CHECK_RET(x)    \
    ret = x;            \
    if (ret != 0) {		\
        return ret;     \
    }

enum
{
    KABLETOP_SCRIPT_ERROR = 4,
    KABLETOP_ARGS_FORMAT_ERROR,
    KABLETOP_ROUND_FORMAT_ERROR,
    KABLETOP_EXCESSIVE_ROUNDS,
    KABLETOP_EXCESSIVE_WITNESS_BYTES,
};

typedef struct
{
    uint8_t  size;
    uint8_t *code;
} Operation;

typedef struct
{
    // from witnesses
    uint8_t round_count;
    mol_seg_t rounds[MAX_ROUND_COUNT];
} Kabletop;

uint8_t _operations_count(Kabletop *k, uint8_t i)
{
    mol_seg_t operations = MolReader_Round_get_operations(&k->rounds[i]);
    return (uint8_t)MolReader_Operations_length(&operations);
}

Operation _operation(Kabletop *k, uint8_t r, uint8_t i)
{
    mol_seg_t operation = MolReader_Round_get_operations(&k->rounds[r]);
    operation = MolReader_Operations_get(&operation, i).seg;
    Operation op;
    op.size = (uint8_t)MolReader_bytes_length(&operation);
    op.code = (uint8_t *)MolReader_bytes_raw_bytes(&operation).ptr;
    return op;
}

int extract_witness_input_type(uint8_t *witness, uint64_t len, mol_seg_t *input_type_bytes_seg)
{
	mol_seg_t witness_seg;
	witness_seg.ptr = witness;
	witness_seg.size = len;

	if (MolReader_WitnessArgs_verify(&witness_seg, false) != MOL_OK)
	{
		return ERROR_ENCODING;
	}
	mol_seg_t input_type_seg = MolReader_WitnessArgs_get_input_type(&witness_seg);

	if (MolReader_BytesOpt_is_none(&input_type_seg))
	{
		return ERROR_ENCODING;
	}
	*input_type_bytes_seg = MolReader_Bytes_raw_bytes(&input_type_seg);
	return CKB_SUCCESS;
}

void print_hex(const char *prefix, unsigned char *msg, int size)
{
	char debug[1024] = "";
	char x[16];
	int j = 0;
	for (int i = 0; i < size; ++i)
	{
		sprintf(x, "%02x", (int)msg[i]);
		memcpy(&debug[j], x, strlen(x));
		j += strlen(x);
	}
	char print[2048];
	sprintf(print, "%s = \"%s\"", prefix, debug);
	ckb_debug(print);
}

int verify_witnesses(Kabletop *kabletop, uint8_t witnesses[MAX_ROUND_COUNT][MAX_ROUND_SIZE])
{
    int ret = CKB_SUCCESS;
    int n = 0;
    uint64_t len = MAX_ROUND_SIZE;
    while (ckb_load_witness(witnesses[n], &len, 0, n, CKB_SOURCE_INPUT) != CKB_INDEX_OUT_OF_BOUND)
    {
        if (len > MAX_ROUND_SIZE)
        {
            return KABLETOP_EXCESSIVE_WITNESS_BYTES;
        }
        n += 1;
    }
    kabletop->round_count = n;
    if (kabletop->round_count > MAX_ROUND_COUNT || kabletop->round_count == 0)
    {
        return KABLETOP_EXCESSIVE_ROUNDS;
    }

    for (uint8_t i = 0; i < kabletop->round_count; ++i)
    {
        uint8_t *witness = witnesses[i];
        len = MAX_ROUND_SIZE;
        ckb_load_witness(witness, &len, 0, i, CKB_SOURCE_INPUT);
        // extract round signature from extra witness lock
        CHECK_RET(extract_witness_input_type(witness, len, &kabletop->rounds[i]));
        if (MolReader_Round_verify(&kabletop->rounds[i], false) != MOL_OK
            || _operations_count(kabletop, i) > MAX_OPERATIONS_PER_ROUND)
        {
            return KABLETOP_ROUND_FORMAT_ERROR;
        }
    }
    return CKB_SUCCESS;
}

#endif
