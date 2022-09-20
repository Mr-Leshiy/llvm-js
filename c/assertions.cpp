#include <assert.h>
#include <stdlib.h>

#include "assertions.hpp"
#include "logical.hpp"
#include "variable.hpp"

void variable_assert(Variable *val)
{
    assert(val != NULL);

    Variable *ret = convert_to_boolean(val);
    if (!ret->boolean_field)
    {
        abort();
    }
}

void variable_assert_eq(Variable *val1, Variable *val2)
{
    assert(val1 != NULL);
    assert(val2 != NULL);

    Variable *ret = logical_seq(val1, val2);
    if (!ret->boolean_field)
    {
        abort();
    }
}