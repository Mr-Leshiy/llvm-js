#ifndef C_NUMBER_HPP
#define C_NUMBER_HPP

#include "variable/variable.hpp"

enum class NumberType
{
    NaN,
    Infinity,
    NegInfinity,
    Number,
};

struct Number
{
    Number(NumberType type) : value(0), type(type) {}
    Number(double value) : value(value), type(NumberType::Number) {}

    NumberType get_type() const;
    double get_value() const;
    std::string to_string() const;

    friend Number operator+(const Number &a, const Number &b);
    friend Number operator-(const Number &a, const Number &b);
    friend Number operator*(const Number &a, const Number &b);
    friend Number operator/(const Number &a, const Number &b);

private:
    double value;
    NumberType type;
};

#endif