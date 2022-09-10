#include <assert.h>
#include <stdlib.h>
#include <string.h>

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
    assert(val1 != NULL);
    assert(val2 != NULL);

    VariableType* ret = allocate();
    val1 = convert_to_boolean(val1);
    val2 = convert_to_boolean(val2);
    
    set_boolean(ret, val1->boolean_field && val2->boolean_field);
    return ret;
}

VariableType* logical_or(VariableType* val1, VariableType* val2)
{
    assert(val1 != NULL);
    assert(val2 != NULL);

    VariableType* ret = allocate();
    val1 = convert_to_boolean(val1);
    val2 = convert_to_boolean(val2);
    
    set_boolean(ret, val1->boolean_field || val2->boolean_field);
    return ret;
}

VariableType* logical_eq(VariableType* val1, VariableType* val2)
{
    // TODO implement
    return logical_seq(val1, val2);
}

VariableType* logical_ne(VariableType* val1, VariableType* val2)
{
    // TODO implement
    return logical_sne(val1, val2);
}

VariableType* logical_seq(VariableType* val1, VariableType* val2)
{
    assert(val1 != NULL);
    assert(val2 != NULL);

    VariableType* ret = allocate();

    switch (val1->flag)
    {
        case Number:
            if(val2->flag == Number)
            {
                set_boolean(ret, val1->number_field == val2->number_field);
            }
            else
            {
                set_boolean(ret, 0);
            }
            break;
        case Boolean:
            if(val2->flag == Boolean)
            {
                set_boolean(ret, val1->boolean_field == val2->boolean_field);
            }
            else
            {
                set_boolean(ret, 0);
            }
            break;
        case String:
            if(val2->flag == String)
            {
                set_boolean(ret, strcmp(val1->string_field, val2->string_field) == 0);
            }
            else
            {
                set_boolean(ret, 0);
            }
            break;
        default:
            assert(0);
            break;
    }
    return ret;
}

VariableType* logical_sne(VariableType* val1, VariableType* val2)
{
    assert(val1 != NULL);
    assert(val2 != NULL);

    VariableType* ret = logical_seq(val1, val2);
    ret->boolean_field = !ret->boolean_field;
    return ret;
}