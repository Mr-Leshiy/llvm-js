#ifndef C_VARIABLE_HPP
#define C_VARIABLE_HPP

#include <string>

#include "object.hpp"
#include "array.hpp"

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
    Array,
};

struct Variable
{
    Variable();
    Variable &operator=(const Variable &);

    void set_undefined();
    void set_null();
    void set_nan();
    void set_infinity();
    void set_neginfinity();
    void set_number(double);
    void set_boolean(bool);
    void set_string(std::string);
    void set_object(const Object &);
    void set_variable(const Variable &);

    bool to_boolean() const;
    Variable* to_number() const;
    std::string to_string() const;

    Type flag;
    double number_field;
    bool boolean_field;
    std::string string_field;
    Object object_field;
    Array array_field;
};

#endif