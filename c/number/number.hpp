#ifndef C_NUMBER_HPP
#define C_NUMBER_HPP

enum class NumberType
{
    NaN,
    Infinity,
    NegInfinity,
    Number,
};

struct Number
{
    Number() = default;
    Number(NumberType type) : value(0), type(type) {}
    Number(double value) : value(value), type(NumberType::Number) {}

    NumberType get_type() const;
    double get_value() const;
    bool to_boolean() const;
    std::string to_string() const;

    friend Number operator+(const Number &a, const Number &b);
    friend Number operator-(const Number &a, const Number &b);
    friend Number operator*(const Number &a, const Number &b);
    friend Number operator/(const Number &a, const Number &b);
    
    friend bool operator==(const Number &a, const Number &b);
    friend bool operator!=(const Number &a, const Number &b);

private:
    double value;
    NumberType type;
};

#endif