#include "kabletop/core.h"

int main()
{
    // molecule buffers
    uint8_t witnesses[MAX_ROUND_SIZE][MAX_ROUND_COUNT];

    Kabletop kabletop;
    int ret = CKB_SUCCESS;

    // recover kabletop rounds from witnesses
    CHECK_RET(verify_witnesses(&kabletop, witnesses));

    // check lua operations
    for (uint8_t i = 0; i < kabletop.round_count; ++i)
    {
        uint8_t count = _operations_count(&kabletop, i);
        for (uint8_t n = 0; n < count; ++n)
        {
            Operation operation = _operation(&kabletop, i, n);
			print_hex("   C", operation.code, operation.size);
        }
    }

    return ret;
}
