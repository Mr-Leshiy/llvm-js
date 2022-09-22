#ifndef C_VARIABLE_HPP
#define C_VARIABLE_HPP

#include <string>

extern "C"
{
#include "variable.h"
}

enum class Type
{
    Undefined,
    Null,
    NaN,
    Infinity,
    NegInfinity,
    Number,
    Boolean,
    String,
};

struct Variable
{
    Type flag;
    double number_field;
    bool boolean_field;
    std::string string_field;
};

#endif