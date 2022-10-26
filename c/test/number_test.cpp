#include <gtest/gtest.h>

#include "number/number.hpp"

TEST(Number, Basic_test)
{
    Number val(123);
    EXPECT_EQ(val.get_type(), NumberType::Number);
    EXPECT_EQ(val.get_value(), 123);
    EXPECT_EQ(val.to_string(), "123.000000");

    val = Number(NumberType::NaN);
    EXPECT_EQ(val.get_type(), NumberType::NaN);
    EXPECT_EQ(val.get_value(), 0);
    EXPECT_EQ(val.to_string(), "NaN");

    val = Number(NumberType::Infinity);
    EXPECT_EQ(val.get_type(), NumberType::Infinity);
    EXPECT_EQ(val.get_value(), 0);
    EXPECT_EQ(val.to_string(), "Infinity");

    val = Number(NumberType::NegInfinity);
    EXPECT_EQ(val.get_type(), NumberType::NegInfinity);
    EXPECT_EQ(val.get_value(), 0);
    EXPECT_EQ(val.to_string(), "-Infinity");

    val = Number(NumberType::Number);
    EXPECT_EQ(val.get_type(), NumberType::Number);
    EXPECT_EQ(val.get_value(), 0);
    EXPECT_EQ(val.to_string(), "0.000000");
}

TEST(Number, addition_test)
{
    Number a(0);
    Number b(0);
    Number res(0);

    // 2 + 3 = 5
    a = Number(2);
    b = Number(3);
    res = a + b;
    EXPECT_EQ(res.get_type(), NumberType::Number);
    EXPECT_EQ(res.get_value(), 5);

    // 0 + 2 = 2
    a = Number(0);
    b = Number(2);
    res = a + b;
    EXPECT_EQ(res.get_type(), NumberType::Number);
    EXPECT_EQ(res.get_value(), 2);

    // -2 + 3 = 1
    a = Number(-2);
    b = Number(3);
    res = a + b;
    EXPECT_EQ(res.get_type(), NumberType::Number);
    EXPECT_EQ(res.get_value(), 1);

    // -2.5 + -4.5 = -7
    a = Number(-2.5);
    b = Number(-4.5);
    res = a + b;
    EXPECT_EQ(res.get_type(), NumberType::Number);
    EXPECT_EQ(res.get_value(), -7);

    // NaN + -4.5 = NaN
    a = Number(NumberType::NaN);
    b = Number(-4.5);
    res = a + b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);

    // -4.5 + NaN = NaN
    a = Number(-4.5);
    b = Number(NumberType::NaN);
    res = a + b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);

    // Infinity + NaN = NaN
    a = Number(NumberType::Infinity);
    b = Number(NumberType::NaN);
    res = a + b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);

    // -Infinity + NaN = NaN
    a = Number(NumberType::NegInfinity);
    b = Number(NumberType::NaN);
    res = a + b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);

    // Infinity + -4.5 = Infinity
    a = Number(NumberType::Infinity);
    b = Number(-4.5);
    res = a + b;
    EXPECT_EQ(res.get_type(), NumberType::Infinity);

    // -Infinity + -4.5 = Infinity
    a = Number(NumberType::NegInfinity);
    b = Number(-4.5);
    res = a + b;
    EXPECT_EQ(res.get_type(), NumberType::NegInfinity);

    // Infinity + -Infinity = NaN
    a = Number(NumberType::Infinity);
    b = Number(NumberType::NegInfinity);
    res = a + b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);

    // -Infinity + Infinity = NaN
    a = Number(NumberType::NegInfinity);
    b = Number(NumberType::Infinity);
    res = a + b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);

    // Infinity + Infinity = Infinity
    a = Number(NumberType::Infinity);
    b = Number(NumberType::Infinity);
    res = a + b;
    EXPECT_EQ(res.get_type(), NumberType::Infinity);

    // -Infinity + -Infinity = -Infinity
    a = Number(NumberType::NegInfinity);
    b = Number(NumberType::NegInfinity);
    res = a + b;
    EXPECT_EQ(res.get_type(), NumberType::NegInfinity);
}