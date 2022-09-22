#include <assert.h>

#include "arithmetic.hpp"
#include "variable.hpp"

Variable *arithmetic_addition(Variable *val1, Variable *val2)
{
    assert(val1 != nullptr);
    assert(val2 != nullptr);

    Variable *ret = allocate();
    val1 = convert_to_number(val1);
    val2 = convert_to_number(val2);

    if (val1->flag == Type::NaN || val2->flag == Type::NaN)
    {
        set_nan(ret);
    }
    else if (val1->flag == Type::Infinity)
    {
        if (val2->flag == Type::NegInfinity)
        {
            set_nan(ret);
        }
        else
        {
            set_infinity(ret);
        }
    }
    else if (val2->flag == Type::Infinity)
    {
        if (val1->flag == Type::NegInfinity)
        {
            set_nan(ret);
        }
        else
        {
            set_infinity(ret);
        }
    }
    else if (val1->flag == Type::NegInfinity)
    {
        if (val2->flag == Type::Infinity)
        {
            set_nan(ret);
        }
        else
        {
            set_neginfinity(ret);
        }
    }
    else if (val2->flag == Type::NegInfinity)
    {
        if (val1->flag == Type::Infinity)
        {
            set_nan(ret);
        }
        else
        {
            set_neginfinity(ret);
        }
    }
    else
    {
        set_number(ret, val1->number_field + val2->number_field);
    }
    return ret;
}

Variable *arithmetic_substraction(Variable *val1, Variable *val2)
{
    assert(val1 != nullptr);
    assert(val2 != nullptr);

    Variable *ret = allocate();
    val1 = convert_to_number(val1);
    val2 = convert_to_number(val2);

    if (val1->flag == Type::NaN || val2->flag == Type::NaN)
    {
        set_nan(ret);
    }
    else if (val1->flag == Type::Infinity)
    {
        if (val2->flag == Type::NegInfinity)
        {
            set_infinity(ret);
        }
        else if (val2->flag == Type::Infinity)
        {
            set_nan(ret);
        }
        else
        {
            set_infinity(ret);
        }
    }
    else if (val2->flag == Type::Infinity)
    {
        if (val1->flag == Type::NegInfinity)
        {
            set_neginfinity(ret);
        }
        else if (val1->flag == Type::Infinity)
        {
            set_nan(ret);
        }
        else
        {
            set_neginfinity(ret);
        }
    }
    else if (val1->flag == Type::NegInfinity)
    {
        if (val2->flag == Type::NegInfinity)
        {
            set_nan(ret);
        }
        else if (val2->flag == Type::Infinity)
        {
            set_neginfinity(ret);
        }
        else
        {
            set_neginfinity(ret);
        }
    }
    else if (val2->flag == Type::NegInfinity)
    {
        if (val1->flag == Type::NegInfinity)
        {
            set_nan(ret);
        }
        else if (val1->flag == Type::Infinity)
        {
            set_infinity(ret);
        }
        else
        {
            set_infinity(ret);
        }
    }
    else
    {
        set_number(ret, val1->number_field - val2->number_field);
    }
    return ret;
}

Variable *arithmetic_multiplication(Variable *val1, Variable *val2)
{
    assert(val1 != nullptr);
    assert(val2 != nullptr);

    Variable *ret = allocate();
    val1 = convert_to_number(val1);
    val2 = convert_to_number(val2);

    if (val1->flag == Type::NaN || val2->flag == Type::NaN)
    {
        set_nan(ret);
    }
    else if (val1->flag == Type::Infinity)
    {
        if (val2->flag == Type::NegInfinity)
        {
            set_neginfinity(ret);
        }
        else if (val2->flag == Type::Infinity)
        {
            set_infinity(ret);
        }
        else
        {
            if (val2->number_field == 0)
            {
                set_nan(ret);
            }
            else if (val2->number_field > 0)
            {
                set_infinity(ret);
            }
            else
            {
                set_neginfinity(ret);
            }
        }
    }
    else if (val2->flag == Type::Infinity)
    {
        if (val1->flag == Type::NegInfinity)
        {
            set_neginfinity(ret);
        }
        else if (val1->flag == Type::Infinity)
        {
            set_infinity(ret);
        }
        else
        {
            if (val1->number_field == 0)
            {
                set_nan(ret);
            }
            else if (val1->number_field > 0)
            {
                set_infinity(ret);
            }
            else
            {
                set_neginfinity(ret);
            }
        }
    }
    else if (val1->flag == Type::NegInfinity)
    {
        if (val2->flag == Type::NegInfinity)
        {
            set_infinity(ret);
        }
        else if (val2->flag == Type::Infinity)
        {
            set_neginfinity(ret);
        }
        else
        {
            if (val2->number_field == 0)
            {
                set_nan(ret);
            }
            else if (val2->number_field > 0)
            {
                set_neginfinity(ret);
            }
            else
            {
                set_infinity(ret);
            }
        }
    }
    else if (val2->flag == Type::NegInfinity)
    {
        if (val1->flag == Type::NegInfinity)
        {
            set_infinity(ret);
        }
        else if (val1->flag == Type::Infinity)
        {
            set_neginfinity(ret);
        }
        else
        {
            if (val1->number_field == 0)
            {
                set_nan(ret);
            }
            else if (val1->number_field > 0)
            {
                set_neginfinity(ret);
            }
            else
            {
                set_infinity(ret);
            }
        }
    }
    else
    {
        set_number(ret, val1->number_field * val2->number_field);
    }
    return ret;
}

Variable *arithmetic_division(Variable *val1, Variable *val2)
{
    assert(val1 != nullptr);
    assert(val2 != nullptr);

    Variable *ret = allocate();
    val1 = convert_to_number(val1);
    val2 = convert_to_number(val2);

    if (val1->flag == Type::NaN || val2->flag == Type::NaN)
    {
        set_nan(ret);
    }
    else if (val1->flag == Type::Infinity)
    {
        if (val2->flag == Type::NegInfinity || val2->flag == Type::Infinity)
        {
            set_nan(ret);
        }
        else
        {
            if (val2->number_field >= 0)
            {
                set_infinity(ret);
            }
            else
            {
                set_neginfinity(ret);
            }
        }
    }
    else if (val2->flag == Type::Infinity)
    {
        if (val1->flag == Type::NegInfinity || val1->flag == Type::Infinity)
        {
            set_nan(ret);
        }
        else
        {
            set_number(ret, 0);
        }
    }
    else if (val1->flag == Type::NegInfinity)
    {
        if (val2->flag == Type::NegInfinity || val2->flag == Type::Infinity)
        {
            set_nan(ret);
        }
        else
        {
            if (val2->number_field >= 0)
            {
                set_neginfinity(ret);
            }
            else
            {
                set_infinity(ret);
            }
        }
    }
    else if (val2->flag == Type::NegInfinity)
    {
        if (val1->flag == Type::NegInfinity || val1->flag == Type::Infinity)
        {
            set_nan(ret);
        }
        else
        {
            set_number(ret, 0);
        }
    }
    else
    {
        if (val2->number_field != 0)
        {
            set_number(ret, val1->number_field / val2->number_field);
        }
        else if (val1->number_field == 0)
        {
            set_nan(ret);
        }
        else if (val1->number_field > 0)
        {
            set_infinity(ret);
        }
        else
        {
            set_neginfinity(ret);
        }
    }
    return ret;
}