#ifndef C_VARIABLE_HPP
#define C_VARIABLE_HPP

#include <string>

#include "object.hpp"

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
    Object,
};

struct Variable
{
    std::string to_string() const;
    void set_object(Object &object);

    Type flag;
    double number_field;
    bool boolean_field;
    std::string string_field;
    Object object_field;
};

#endif