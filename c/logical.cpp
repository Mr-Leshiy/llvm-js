#include <assert.h>
#include <stdlib.h>
#include <string.h>

#include "logical.hpp"
#include "variable.hpp"

Variable *logical_not(Variable *val)
{
    assert(val != nullptr);

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
    assert(val1 != nullptr);
    assert(val2 != nullptr);

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
    assert(val1 != nullptr);
    assert(val2 != nullptr);

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
    assert(val1 != nullptr);
    assert(val2 != nullptr);

    Variable *ret = allocate();

    if (val1 == val2)
    {
        set_boolean(ret, true);
        return ret;
    }

    switch (val1->flag)
    {
    case Type::Undefined:
        set_boolean(ret, val2->flag == Type::Undefined);
        break;
    case Type::Null:
        set_boolean(ret, val2->flag == Type::Null);
        break;
    case Type::NaN:
        set_boolean(ret, val2->flag == Type::NaN);
        break;
    case Type::Infinity:
        set_boolean(ret, val2->flag == Type::Infinity);
        break;
    case Type::NegInfinity:
        set_boolean(ret, val2->flag == Type::NegInfinity);
        break;
    case Type::Number:
        if (val2->flag == Type::Number)
        {
            set_boolean(ret, val1->number_field == val2->number_field);
        }
        else
        {
            set_boolean(ret, false);
        }
        break;
    case Type::Boolean:
        if (val2->flag == Type::Boolean)
        {
            set_boolean(ret, val1->boolean_field == val2->boolean_field);
        }
        else
        {
            set_boolean(ret, false);
        }
        break;
    case Type::String:
        if (val2->flag == Type::String)
        {
            set_boolean(ret, val1->string_field == val2->string_field);
        }
        else
        {
            set_boolean(ret, false);
        }
        break;
    case Type::Object:
        set_boolean(ret, false);
        break;
    default:
        assert(0);
        break;
    }
    return ret;
}

Variable *logical_sne(Variable *val1, Variable *val2)
{
    assert(val1 != nullptr);
    assert(val2 != nullptr);

    Variable *ret = logical_seq(val1, val2);
    ret->boolean_field = !ret->boolean_field;
    return ret;
}