#ifndef C_VARIABLE_HPP
#define C_VARIABLE_HPP

#include <string>

#include "object/object.hpp"
#include "number/number.hpp"
#include "array.hpp"

extern "C"
{
#include "variable.h"
}

enum class Type
{
    Undefined,
    Null,
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
    void set_number(Number);
    void set_boolean(bool);
    void set_string(std::string);
    void set_object(const Object &);

    bool to_boolean() const;
    Number to_number() const;
    std::string to_string() const;

    Type flag;
    Number number_field;
    bool boolean_field;
    std::string string_field;
    Object object_field;
    Array array_field;
};

#endif