#include <assert.h>
#include <stdlib.h>
#include <string.h>

#include "logical.hpp"
#include "variable.hpp"

Variable *logical_not(Variable *val)
{
    assert(val != NULL);

    Variable *ret = convert_to_boolean(val);
    if (ret->boolean_field)
    {
        ret->boolean_field = 0;
    }
    else
    {
        ret->boolean_field = 1;
    }
    return ret;
}

Variable *logical_and(Variable *val1, Variable *val2)
{
    assert(val1 != NULL);
    assert(val2 != NULL);

    Variable *ret = allocate();
    Variable *val1_ = convert_to_boolean(val1);
    Variable *val2_ = convert_to_boolean(val2);

    if (val1_->boolean_field)
    {
        if (val2_->boolean_field)
        {
            set_variable(ret, val2);
        }
        else
        {
            set_variable(ret, val2);
        }
    }
    else
    {
        set_variable(ret, val1);
    }
    return ret;
}

Variable *logical_or(Variable *val1, Variable *val2)
{
    assert(val1 != NULL);
    assert(val2 != NULL);

    Variable *ret = allocate();
    Variable *val1_ = convert_to_boolean(val1);
    Variable *val2_ = convert_to_boolean(val2);

    if (val1_->boolean_field)
    {
        set_variable(ret, val1);
    }
    else
    {
        if (val2_->boolean_field)
        {
            set_variable(ret, val2);
        }
        else
        {
            set_variable(ret, val2);
        }
    }
    return ret;
}

Variable *logical_eq(Variable *val1, Variable *val2)
{
    // TODO implement
    return logical_seq(val1, val2);
}

Variable *logical_ne(Variable *val1, Variable *val2)
{
    // TODO implement
    return logical_sne(val1, val2);
}

Variable *logical_seq(Variable *val1, Variable *val2)
{
    assert(val1 != NULL);
    assert(val2 != NULL);

    Variable *ret = allocate();

    switch (val1->flag)
    {
    case Type::Undefined:
        if (val2->flag == Type::Undefined)
        {
            set_boolean(ret, 1);
        }
        else
        {
            set_boolean(ret, 0);
        }
        break;
    case Type::Null:
        if (val2->flag == Type::Null)
        {
            set_boolean(ret, 1);
        }
        else
        {
            set_boolean(ret, 0);
        }
        break;
    case Type::NaN:
        if (val2->flag == Type::NaN)
        {
            set_boolean(ret, 1);
        }
        else
        {
            set_boolean(ret, 0);
        }
        break;
    case Type::Number:
        if (val2->flag == Type::Number)
        {
            set_boolean(ret, val1->number_field == val2->number_field);
        }
        else
        {
            set_boolean(ret, 0);
        }
        break;
    case Type::Boolean:
        if (val2->flag == Type::Boolean)
        {
            set_boolean(ret, val1->boolean_field == val2->boolean_field);
        }
        else
        {
            set_boolean(ret, 0);
        }
        break;
    case Type::String:
        if (val2->flag == Type::String)
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

Variable *logical_sne(Variable *val1, Variable *val2)
{
    assert(val1 != NULL);
    assert(val2 != NULL);

    Variable *ret = logical_seq(val1, val2);
    ret->boolean_field = !ret->boolean_field;
    return ret;
}