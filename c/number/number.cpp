#include "number.hpp"

NumberType Number::get_type() const
{
    return this->type;
}

double Number::get_value() const
{
    return this->value;
}

std::string Number::to_string() const
{
    switch (this->type)
    {
    case NumberType::NaN:
        return "NaN";
        break;
    case NumberType::Infinity:
        return "Infinity";
        break;
    case NumberType::NegInfinity:
        return "-Infinity";
        break;
    case NumberType::Number:
        return std::to_string(this->value);
        break;
    default:
        assert(false);
        break;
    }
}

Number operator+(const Number &a, const Number &b)
{
    Number ret(0);

    if (a.type == NumberType::NaN || b.type == NumberType::NaN)
    {
        ret.type = NumberType::NaN;
    }
    else if (a.type == NumberType::Infinity)
    {
        if (b.type == NumberType::NegInfinity)
        {
            ret.type = NumberType::NaN;
        }
        else
        {
            ret.type = NumberType::Infinity;
        }
    }
    else if (b.type == NumberType::Infinity)
    {
        if (a.type == NumberType::NegInfinity)
        {
            ret.type = NumberType::NaN;
        }
        else
        {
            ret.type = NumberType::Infinity;
        }
    }
    else if (a.type == NumberType::NegInfinity)
    {
        if (b.type == NumberType::Infinity)
        {
            ret.type = NumberType::NaN;
        }
        else
        {
            ret.type = NumberType::NegInfinity;
        }
    }
    else if (b.type == NumberType::NegInfinity)
    {
        if (a.type == NumberType::Infinity)
        {
            ret.type = NumberType::NaN;
        }
        else
        {
            ret.type = NumberType::NegInfinity;
        }
    }
    else
    {
        ret.value = a.value + b.value;
    }

    return ret;
}

Number operator-(const Number &a, const Number &b)
{
    Number ret(0);

    return ret;
}

Number operator*(const Number &a, const Number &b)
{
    Number ret(0);

    return ret;
}

Number operator/(const Number &a, const Number &b)
{
    Number ret(0);

    return ret;
}