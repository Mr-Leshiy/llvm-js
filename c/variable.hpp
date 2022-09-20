#ifndef C_VARIABLE_HPP
#define C_VARIABLE_HPP

#include <string>

extern "C"
{
#include "variable.h"
}

enum class Type
{
    Undefined = 0,
    Null = 1,
    NaN = 2,
    Number = 3,
    Boolean = 4,
    String = 5,
};

struct Variable
{
    Type flag;
    double number_field;
    bool boolean_field;
    std::string string_field;
};

#endif