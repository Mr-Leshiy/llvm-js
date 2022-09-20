#include "arithmetic.hpp"
#include "variable.hpp"

Variable *arithmetic_addition(Variable *val1, Variable *val2)
{
    assert(val1 != NULL);
    assert(val2 != NULL);

    Variable *ret = allocate();
    val1 = convert_to_number(val1);
    val2 = convert_to_number(val2);

    if (val1->flag == Type::NaN || val2->flag == Type::NaN)
    {
        set_nan(ret);
    }
    else
    {
        set_number(ret, val1->number_field + val2->number_field);
    }
    return ret;
}

Variable *arithmetic_substraction(Variable *val1, Variable *val2)
{
    assert(val1 != NULL);
    assert(val2 != NULL);

    Variable *ret = allocate();
    val1 = convert_to_number(val1);
    val2 = convert_to_number(val2);

    if (val1->flag == Type::NaN || val2->flag == Type::NaN)
    {
        set_nan(ret);
    }
    else
    {
        set_number(ret, val1->number_field - val2->number_field);
    }
    return ret;
}

Variable *arithmetic_multiplication(Variable *val1, Variable *val2)
{
    assert(val1 != NULL);
    assert(val2 != NULL);

    Variable *ret = allocate();
    val1 = convert_to_number(val1);
    val2 = convert_to_number(val2);

    if (val1->flag == Type::NaN || val2->flag == Type::NaN)
    {
        set_nan(ret);
    }
    else
    {
        set_number(ret, val1->number_field * val2->number_field);
    }
    return ret;
}