#ifndef C_VARIABLE_HPP
#define C_VARIABLE_HPP

#include <string>

#include "object/object.hpp"
#include "number/number.hpp"
#include "array/array.hpp"

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

    Type get_flag() const;
    bool get_boolean() const;
    const std::string& get_string() const;
    const Number& get_number() const;

    bool to_boolean() const;
    Number to_number() const;
    std::string to_string() const;

    friend Variable operator+(const Variable &a, const Variable &b);
    friend Variable operator-(const Variable &a, const Variable &b);
    friend Variable operator*(const Variable &a, const Variable &b);
    friend Variable operator/(const Variable &a, const Variable &b);

    friend bool operator!(const Variable &a);
    friend bool operator==(const Variable &a, const Variable &b);
    friend bool operator!=(const Variable &a, const Variable &b);
    friend Variable operator&&(const Variable &a, const Variable &b);
    friend Variable operator||(const Variable &a, const Variable &b);

    Object object_field;

private:
    Type flag;
    Number number_field;
    bool boolean_field;
    std::string string_field;
    Array array_field;
};

#endif