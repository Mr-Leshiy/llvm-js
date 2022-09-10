#include <assert.h>
#include <stdlib.h>

#include "logical.h"

VariableType* logical_not(VariableType* val)
{
    assert(val != NULL);

    VariableType* ret = convert_to_boolean(val);
    if(ret->boolean_field)
    {
        ret->boolean_field = 0;
    }
    else
    {
        ret->boolean_field = 1;
    }
    return ret;
}

VariableType* logical_and(VariableType* val1, VariableType* val2)
{

}

VariableType* logical_or(VariableType* val1, VariableType* val2)
{

}

VariableType* logical_eq(VariableType* val1, VariableType* val2)
{

}

VariableType* logical_ne(VariableType* val1, VariableType* val2)
{

}

VariableType* logical_seq(VariableType* val1, VariableType* val2)
{

}

VariableType* logical_sne(VariableType* val1, VariableType* val2)
{

}