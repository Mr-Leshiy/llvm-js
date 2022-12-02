#include <string>

#include "assert/assert.hpp"
#include "number.hpp"

NumberType Number::get_type() const
{
    return this->type;
}

double Number::get_value() const
{
    return this->value;
}

bool Number::to_boolean() const
{
    switch (this->type)
    {
    case NumberType::NaN:
        return false;
        break;
    case NumberType::Infinity:
        return true;
        break;
    case NumberType::NegInfinity:
        return true;
        break;
    case NumberType::Number:
        return this->value != 0;
        break;
    default:
        ASSERT(false);
        break;
    }
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
        ASSERT(false);
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

    if (a.type == NumberType::NaN || b.type == NumberType::NaN)
    {
        ret.type = NumberType::NaN;
    }
    else if (a.type == NumberType::Infinity)
    {
        if (b.type == NumberType::NegInfinity)
        {
            ret.type = NumberType::Infinity;
        }
        else if (b.type == NumberType::Infinity)
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
            ret.type = NumberType::NegInfinity;
        }
        else if (a.type == NumberType::Infinity)
        {
            ret.type = NumberType::NaN;
        }
        else
        {
            ret.type = NumberType::NegInfinity;
        }
    }
    else if (a.type == NumberType::NegInfinity)
    {
        if (b.type == NumberType::NegInfinity)
        {
            ret = NumberType::NaN;
        }
        else if (b.type == NumberType::Infinity)
        {
            ret = NumberType::NegInfinity;
        }
        else
        {
            ret = NumberType::NegInfinity;
        }
    }
    else if (b.type == NumberType::NegInfinity)
    {
        if (a.type == NumberType::NegInfinity)
        {
            ret = NumberType::NaN;
        }
        else if (a.type == NumberType::Infinity)
        {
            ret = NumberType::Infinity;
        }
        else
        {
            ret = NumberType::Infinity;
        }
    }
    else
    {
        ret.value = a.value - b.value;
    }

    return ret;
}

Number operator*(const Number &a, const Number &b)
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
            ret.type = NumberType::NegInfinity;
        }
        else if (b.type == NumberType::Infinity)
        {
            ret.type = NumberType::Infinity;
        }
        else
        {
            if (b.value == 0)
            {
                ret.type = NumberType::NaN;
            }
            else if (b.value > 0)
            {
                ret.type = NumberType::Infinity;
            }
            else
            {
                ret.type = NumberType::NegInfinity;
            }
        }
    }
    else if (b.type == NumberType::Infinity)
    {
        if (a.type == NumberType::NegInfinity)
        {
            ret.type = NumberType::NegInfinity;
        }
        else if (a.type == NumberType::Infinity)
        {
            ret.type = NumberType::Infinity;
        }
        else
        {
            if (a.value == 0)
            {
                ret.type = NumberType::NaN;
            }
            else if (a.value > 0)
            {
                ret.type = NumberType::Infinity;
            }
            else
            {
                ret.type = NumberType::NegInfinity;
            }
        }
    }
    else if (a.type == NumberType::NegInfinity)
    {
        if (b.type == NumberType::NegInfinity)
        {
            ret.type = NumberType::Infinity;
        }
        else if (b.type == NumberType::Infinity)
        {
            ret.type = NumberType::NegInfinity;
        }
        else
        {
            if (b.value == 0)
            {
                ret.type = NumberType::NaN;
            }
            else if (b.value > 0)
            {
                ret.type = NumberType::NegInfinity;
            }
            else
            {
                ret.type = NumberType::Infinity;
            }
        }
    }
    else if (b.type == NumberType::NegInfinity)
    {
        if (a.type == NumberType::NegInfinity)
        {
            ret.type = NumberType::Infinity;
        }
        else if (a.type == NumberType::Infinity)
        {
            ret.type = NumberType::NegInfinity;
        }
        else
        {
            if (a.value == 0)
            {
                ret.type = NumberType::NaN;
            }
            else if (a.value > 0)
            {
                ret.type = NumberType::NegInfinity;
            }
            else
            {
                ret.type = NumberType::Infinity;
            }
        }
    }
    else
    {
        ret.value = a.value * b.value;
    }
    return ret;
}

Number operator/(const Number &a, const Number &b)
{
    Number ret(0);

    if (a.type == NumberType::NaN || b.type == NumberType::NaN)
    {
        ret.type = NumberType::NaN;
    }
    else if (a.type == NumberType::Infinity)
    {
        if (b.type == NumberType::NegInfinity || b.type == NumberType::Infinity)
        {
            ret.type = NumberType::NaN;
        }
        else
        {
            if (b.value >= 0)
            {
                ret.type = NumberType::Infinity;
            }
            else
            {
                ret.type = NumberType::NegInfinity;
            }
        }
    }
    else if (b.type == NumberType::Infinity)
    {
        if (a.type == NumberType::NegInfinity || a.type == NumberType::Infinity)
        {
            ret.type = NumberType::NaN;
        }
        else
        {
            ret.value = 0;
        }
    }
    else if (a.type == NumberType::NegInfinity)
    {
        if (b.type == NumberType::NegInfinity || b.type == NumberType::Infinity)
        {
            ret.type = NumberType::NaN;
        }
        else
        {
            if (b.value >= 0)
            {
                ret.type = NumberType::NegInfinity;
            }
            else
            {
                ret.type = NumberType::Infinity;
            }
        }
    }
    else if (b.type == NumberType::NegInfinity)
    {
        if (a.type == NumberType::NegInfinity || a.type == NumberType::Infinity)
        {
            ret.type = NumberType::NaN;
        }
        else
        {
            ret.value = 0;
        }
    }
    else
    {
        if (b.value != 0)
        {
            ret.value = a.value / b.value;
        }
        else if (a.value == 0)
        {
            ret.type = NumberType::NaN;
        }
        else if (a.value > 0)
        {
            ret.type = NumberType::Infinity;
        }
        else
        {
            ret.type = NumberType::NegInfinity;
        }
    }
    return ret;
}

bool operator==(const Number &a, const Number &b)
{
    switch (a.type)
    {
    case NumberType::NaN:
        if (b.type == NumberType::NaN)
        {
            return true;
        }
        break;
    case NumberType::Infinity:
        if (b.type == NumberType::Infinity)
        {
            return true;
        }
        break;
    case NumberType::NegInfinity:
        if (b.type == NumberType::NegInfinity)
        {
            return true;
        }
        break;
    case NumberType::Number:
        if (b.type == NumberType::Number)
        {
            return a.value == b.value;
        }
        break;
    default:
        ASSERT(false);
        break;
    }

    return false;
}

bool operator>(const Number &a, const Number &b)
{
    switch (a.type)
    {
    case NumberType::Number:
        switch (b.type)
        {
        case NumberType::Number:
            return a.value > b.value;
            break;
        case NumberType::Infinity:
            return false;
            break;
        case NumberType::NegInfinity:
            return true;
            break;
        case NumberType::NaN:
            break;
        default:
            ASSERT(false);
            break;
        }
        break;
    case NumberType::Infinity:
        switch (b.type)
        {
        case NumberType::Number:
            return true;
            break;
        case NumberType::Infinity:
            return false;
            break;
        case NumberType::NegInfinity:
            return true;
            break;
        case NumberType::NaN:
            break;
        default:
            ASSERT(false);
            break;
        }
        break;
    case NumberType::NegInfinity:
        switch (b.type)
        {
        case NumberType::Number:
            return false;
            break;
        case NumberType::Infinity:
            return false;
            break;
        case NumberType::NegInfinity:
            return false;
            break;
        case NumberType::NaN:
            break;
        default:
            ASSERT(false);
            break;
        }
        break;
    case NumberType::NaN:
        break;
    default:
        ASSERT(false);
        break;
    }
    return false;
}

bool operator>=(const Number &a, const Number &b)
{
    switch (a.type)
    {
    case NumberType::Number:
        switch (b.type)
        {
        case NumberType::Number:
            return a.value >= b.value;
            break;
        case NumberType::Infinity:
            return false;
            break;
        case NumberType::NegInfinity:
            return true;
            break;
        case NumberType::NaN:
            break;
        default:
            ASSERT(false);
            break;
        }
        break;
    case NumberType::Infinity:
        switch (b.type)
        {
        case NumberType::Number:
            return true;
            break;
        case NumberType::Infinity:
            return true;
            break;
        case NumberType::NegInfinity:
            return true;
            break;
        case NumberType::NaN:
            break;
        default:
            ASSERT(false);
            break;
        }
        break;
    case NumberType::NegInfinity:
        switch (b.type)
        {
        case NumberType::Number:
            return false;
            break;
        case NumberType::Infinity:
            return false;
            break;
        case NumberType::NegInfinity:
            return true;
            break;
        case NumberType::NaN:
            break;
        default:
            ASSERT(false);
            break;
        }
        break;
    case NumberType::NaN:
        break;
    default:
        ASSERT(false);
        break;
    }
    return false;
}

bool operator<(const Number &a, const Number &b)
{
    if (a.type == NumberType::NaN || b.type == NumberType::NaN)
    {
        return false;
    }
    else
    {
        return !(a >= b);
    }
}

bool operator<=(const Number &a, const Number &b)
{
    if (a.type == NumberType::NaN || b.type == NumberType::NaN)
    {
        return false;
    }
    else
    {
        return !(a > b);
    }
}

bool operator!=(const Number &a, const Number &b)
{
    return !(a == b);
}