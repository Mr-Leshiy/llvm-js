#include <assert.h>
#include <stdlib.h>

#include "assertions.h"
#include "logical.h"

void variable_assert(VariableType *val)
{
    assert(val != NULL);

    VariableType *ret = convert_to_boolean(val);
    if (!ret->boolean_field)
    {
        abort();
    }
}

void variable_assert_eq(VariableType *val1, VariableType *val2)
{
    assert(val1 != NULL);
    assert(val2 != NULL);

    VariableType *ret = logical_seq(val1, val2);
    if (!ret->boolean_field)
    {
        abort();
    }
}