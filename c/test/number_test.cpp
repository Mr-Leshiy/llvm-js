#include <gtest/gtest.h>

#include "number/number.hpp"

TEST(Number, Basic_test)
{
    Number val(123);
    EXPECT_EQ(val.get_type(), NumberType::Number);
    EXPECT_EQ(val.get_value(), 123);
    EXPECT_EQ(val.to_string(), "123.000000");
    EXPECT_EQ(val.to_boolean(), true);

    val = Number(NumberType::NaN);
    EXPECT_EQ(val.get_type(), NumberType::NaN);
    EXPECT_EQ(val.get_value(), 0);
    EXPECT_EQ(val.to_string(), "NaN");
    EXPECT_EQ(val.to_boolean(), false);

    val = Number(NumberType::Infinity);
    EXPECT_EQ(val.get_type(), NumberType::Infinity);
    EXPECT_EQ(val.get_value(), 0);
    EXPECT_EQ(val.to_string(), "Infinity");
    EXPECT_EQ(val.to_boolean(), true);

    val = Number(NumberType::NegInfinity);
    EXPECT_EQ(val.get_type(), NumberType::NegInfinity);
    EXPECT_EQ(val.get_value(), 0);
    EXPECT_EQ(val.to_string(), "-Infinity");
    EXPECT_EQ(val.to_boolean(), true);

    val = Number(NumberType::Number);
    EXPECT_EQ(val.get_type(), NumberType::Number);
    EXPECT_EQ(val.get_value(), 0);
    EXPECT_EQ(val.to_string(), "0.000000");
    EXPECT_EQ(val.to_boolean(), false);
}

TEST(Number, eq_test)
{
    EXPECT_EQ(Number(2), Number(2));
    EXPECT_EQ(Number(NumberType::Number), Number(NumberType::Number));
    EXPECT_EQ(Number(NumberType::NaN), Number(NumberType::NaN));
    EXPECT_EQ(Number(NumberType::Infinity), Number(NumberType::Infinity));
    EXPECT_EQ(Number(NumberType::NegInfinity), Number(NumberType::NegInfinity));

    EXPECT_NE(Number(2), Number(0));
    EXPECT_NE(Number(NumberType::Number), Number(NumberType::NaN));
    EXPECT_NE(Number(NumberType::NaN), Number(NumberType::Infinity));
    EXPECT_NE(Number(NumberType::NegInfinity), Number(NumberType::Infinity));
    EXPECT_NE(Number(NumberType::NegInfinity), Number(NumberType::NaN));
}

TEST(Number, cmp_test)
{
    // > tests
    EXPECT_TRUE(Number(2) > Number(-2));
    EXPECT_TRUE(Number(NumberType::Infinity) > Number(2));
    EXPECT_TRUE(Number(NumberType::Infinity) > Number(-2));
    EXPECT_TRUE(Number(NumberType::Infinity) > Number(NumberType::NegInfinity));
    EXPECT_TRUE(Number(2) > Number(NumberType::NegInfinity));
    EXPECT_TRUE(Number(-2) > Number(NumberType::NegInfinity));

    EXPECT_FALSE(Number(2) > Number(NumberType::NaN));
    EXPECT_FALSE(Number(NumberType::NaN) > Number(2));
    EXPECT_FALSE(Number(NumberType::NaN) > Number(NumberType::Infinity));
    EXPECT_FALSE(Number(NumberType::Infinity) > Number(NumberType::NaN));
    EXPECT_FALSE(Number(NumberType::NaN) > Number(NumberType::NegInfinity));
    EXPECT_FALSE(Number(NumberType::NegInfinity) > Number(NumberType::NaN));
    EXPECT_FALSE(Number(NumberType::NaN) > Number(NumberType::NaN));

    EXPECT_FALSE(Number(2) > Number(2));
    EXPECT_FALSE(Number(-2) > Number(2));
    EXPECT_FALSE(Number(NumberType::Infinity) > Number(NumberType::Infinity));
    EXPECT_FALSE(Number(2) > Number(NumberType::Infinity));
    EXPECT_FALSE(Number(-2) > Number(NumberType::Infinity));
    EXPECT_FALSE(Number(NumberType::NegInfinity) > Number(NumberType::Infinity));
    EXPECT_FALSE(Number(NumberType::NegInfinity) > Number(NumberType::NegInfinity));
    EXPECT_FALSE(Number(NumberType::NegInfinity) > Number(2));
    EXPECT_FALSE(Number(NumberType::NegInfinity) > Number(-2));

    // >= tests
    EXPECT_TRUE(Number(2) >= Number(-2));
    EXPECT_TRUE(Number(NumberType::Infinity) >= Number(2));
    EXPECT_TRUE(Number(NumberType::Infinity) >= Number(-2));
    EXPECT_TRUE(Number(NumberType::Infinity) >= Number(NumberType::NegInfinity));
    EXPECT_TRUE(Number(2) >= Number(NumberType::NegInfinity));
    EXPECT_TRUE(Number(-2) >= Number(NumberType::NegInfinity));
    EXPECT_TRUE(Number(2) >= Number(2));
    EXPECT_TRUE(Number(NumberType::Infinity) >= Number(NumberType::Infinity));
    EXPECT_TRUE(Number(NumberType::NegInfinity) >= Number(NumberType::NegInfinity));

    EXPECT_FALSE(Number(2) >= Number(NumberType::NaN));
    EXPECT_FALSE(Number(NumberType::NaN) >= Number(2));
    EXPECT_FALSE(Number(NumberType::NaN) >= Number(NumberType::Infinity));
    EXPECT_FALSE(Number(NumberType::Infinity) >= Number(NumberType::NaN));
    EXPECT_FALSE(Number(NumberType::NaN) >= Number(NumberType::NegInfinity));
    EXPECT_FALSE(Number(NumberType::NegInfinity) >= Number(NumberType::NaN));
    EXPECT_FALSE(Number(NumberType::NaN) >= Number(NumberType::NaN));

    EXPECT_FALSE(Number(-2) >= Number(2));
    EXPECT_FALSE(Number(2) >= Number(NumberType::Infinity));
    EXPECT_FALSE(Number(-2) >= Number(NumberType::Infinity));
    EXPECT_FALSE(Number(NumberType::NegInfinity) >= Number(NumberType::Infinity));
    EXPECT_FALSE(Number(NumberType::NegInfinity) >= Number(2));
    EXPECT_FALSE(Number(NumberType::NegInfinity) >= Number(-2));

    // < tests
    EXPECT_TRUE(Number(-2) < Number(2));
    EXPECT_TRUE(Number(2) < Number(NumberType::Infinity));
    EXPECT_TRUE(Number(-2) < Number(NumberType::Infinity));
    EXPECT_TRUE(Number(NumberType::NegInfinity) < Number(NumberType::Infinity));
    EXPECT_TRUE(Number(NumberType::NegInfinity) < Number(2));
    EXPECT_TRUE(Number(NumberType::NegInfinity) < Number(-2));

    EXPECT_FALSE(Number(NumberType::NaN) < Number(2));
    EXPECT_FALSE(Number(2) < Number(NumberType::NaN));
    EXPECT_FALSE(Number(NumberType::Infinity) < Number(NumberType::NaN));
    EXPECT_FALSE(Number(NumberType::NaN) < Number(NumberType::Infinity));
    EXPECT_FALSE(Number(NumberType::NegInfinity) < Number(NumberType::NaN));
    EXPECT_FALSE(Number(NumberType::NaN) < Number(NumberType::NegInfinity));
    EXPECT_FALSE(Number(NumberType::NaN) < Number(NumberType::NaN));

    EXPECT_FALSE(Number(2) < Number(2));
    EXPECT_FALSE(Number(2) < Number(-2));
    EXPECT_FALSE(Number(NumberType::Infinity) < Number(NumberType::Infinity));
    EXPECT_FALSE(Number(NumberType::Infinity) < Number(2));
    EXPECT_FALSE(Number(NumberType::Infinity) < Number(-2));
    EXPECT_FALSE(Number(NumberType::Infinity) < Number(NumberType::NegInfinity));
    EXPECT_FALSE(Number(NumberType::NegInfinity) < Number(NumberType::NegInfinity));
    EXPECT_FALSE(Number(2) < Number(NumberType::NegInfinity));
    EXPECT_FALSE(Number(-2) < Number(NumberType::NegInfinity));

    // <= tests
    EXPECT_TRUE(Number(-2) <= Number(2));
    EXPECT_TRUE(Number(2) <= Number(NumberType::Infinity));
    EXPECT_TRUE(Number(-2) <= Number(NumberType::Infinity));
    EXPECT_TRUE(Number(NumberType::NegInfinity) <= Number(NumberType::Infinity));
    EXPECT_TRUE(Number(NumberType::NegInfinity) <= Number(2));
    EXPECT_TRUE(Number(NumberType::NegInfinity) <= Number(-2));
    EXPECT_TRUE(Number(2) <= Number(2));
    EXPECT_TRUE(Number(NumberType::Infinity) <= Number(NumberType::Infinity));
    EXPECT_TRUE(Number(NumberType::NegInfinity) <= Number(NumberType::NegInfinity));

    EXPECT_FALSE(Number(NumberType::NaN) <= Number(2));
    EXPECT_FALSE(Number(2) <= Number(NumberType::NaN));
    EXPECT_FALSE(Number(NumberType::Infinity) <= Number(NumberType::NaN));
    EXPECT_FALSE(Number(NumberType::NaN) <= Number(NumberType::Infinity));
    EXPECT_FALSE(Number(NumberType::NegInfinity) <= Number(NumberType::NaN));
    EXPECT_FALSE(Number(NumberType::NaN) <= Number(NumberType::NegInfinity));
    EXPECT_FALSE(Number(NumberType::NaN) <= Number(NumberType::NaN));

    EXPECT_FALSE(Number(2) <= Number(-2));
    EXPECT_FALSE(Number(NumberType::Infinity) <= Number(2));
    EXPECT_FALSE(Number(NumberType::Infinity) <= Number(-2));
    EXPECT_FALSE(Number(NumberType::Infinity) <= Number(NumberType::NegInfinity));
    EXPECT_FALSE(Number(2) <= Number(NumberType::NegInfinity));
    EXPECT_FALSE(Number(-2) <= Number(NumberType::NegInfinity));
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

TEST(Number, substraction_test)
{
    Number a(0);
    Number b(0);
    Number res(0);

    // 2 - 3 = -5
    a = Number(2);
    b = Number(3);
    res = a - b;
    EXPECT_EQ(res.get_type(), NumberType::Number);
    EXPECT_EQ(res.get_value(), -1);

    // 2 - 0 = 2
    a = Number(2);
    b = Number(0);
    res = a - b;
    EXPECT_EQ(res.get_type(), NumberType::Number);
    EXPECT_EQ(res.get_value(), 2);

    // 0 - 2 = -2
    a = Number(0);
    b = Number(2);
    res = a - b;
    EXPECT_EQ(res.get_type(), NumberType::Number);
    EXPECT_EQ(res.get_value(), -2);

    // -2 - 3 = -5
    a = Number(-2);
    b = Number(3);
    res = a - b;
    EXPECT_EQ(res.get_type(), NumberType::Number);
    EXPECT_EQ(res.get_value(), -5);

    // -2.5 - -4.5 = 2
    a = Number(-2.5);
    b = Number(-4.5);
    res = a - b;
    EXPECT_EQ(res.get_type(), NumberType::Number);
    EXPECT_EQ(res.get_value(), 2);

    // NaN - -4.5 = NaN
    a = Number(NumberType::NaN);
    b = Number(-4.5);
    res = a - b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);

    // -4.5 - NaN = NaN
    a = Number(-4.5);
    b = Number(NumberType::NaN);
    res = a - b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);

    // Infinity - NaN = NaN
    a = Number(NumberType::Infinity);
    b = Number(NumberType::NaN);
    res = a - b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);

    // -Infinity - NaN = NaN
    a = Number(NumberType::NegInfinity);
    b = Number(NumberType::NaN);
    res = a - b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);

    // Infinity - -4.5 = Infinity
    a = Number(NumberType::Infinity);
    b = Number(-4.5);
    res = a - b;
    EXPECT_EQ(res.get_type(), NumberType::Infinity);

    // -Infinity - -4.5 = -Infinity
    a = Number(NumberType::NegInfinity);
    b = Number(-4.5);
    res = a - b;
    EXPECT_EQ(res.get_type(), NumberType::NegInfinity);

    // Infinity - -Infinity = Infinity
    a = Number(NumberType::Infinity);
    b = Number(NumberType::NegInfinity);
    res = a - b;
    EXPECT_EQ(res.get_type(), NumberType::Infinity);

    // -Infinity - Infinity = -Infinity
    a = Number(NumberType::NegInfinity);
    b = Number(NumberType::Infinity);
    res = a - b;
    EXPECT_EQ(res.get_type(), NumberType::NegInfinity);

    // Infinity - Infinity = NaN
    a = Number(NumberType::Infinity);
    b = Number(NumberType::Infinity);
    res = a - b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);

    // -Infinity - -Infinity = NaN
    a = Number(NumberType::NegInfinity);
    b = Number(NumberType::NegInfinity);
    res = a - b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);
}

TEST(Number, multiplication_test)
{
    Number a(0);
    Number b(0);
    Number res(0);

    // 2 * 3 = 6
    a = Number(2);
    b = Number(3);
    res = a * b;
    EXPECT_EQ(res.get_type(), NumberType::Number);
    EXPECT_EQ(res.get_value(), 6);

    // 2 * 0 = 0
    a = Number(2);
    b = Number(0);
    res = a * b;
    EXPECT_EQ(res.get_type(), NumberType::Number);
    EXPECT_EQ(res.get_value(), 0);

    // 0 * 2 = 0
    a = Number(0);
    b = Number(2);
    res = a * b;
    EXPECT_EQ(res.get_type(), NumberType::Number);
    EXPECT_EQ(res.get_value(), 0);

    // -2 * 3 = -6
    a = Number(-2);
    b = Number(3);
    res = a * b;
    EXPECT_EQ(res.get_type(), NumberType::Number);
    EXPECT_EQ(res.get_value(), -6);

    // -2.5 * -4.5 = 11.25
    a = Number(-2.5);
    b = Number(-4.5);
    res = a * b;
    EXPECT_EQ(res.get_type(), NumberType::Number);
    EXPECT_EQ(res.get_value(), 11.25);

    // NaN * -4.5 = NaN
    a = Number(NumberType::NaN);
    b = Number(-4.5);
    res = a * b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);

    // -4.5 * NaN = NaN
    a = Number(-4.5);
    b = Number(NumberType::NaN);
    res = a * b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);

    // Infinity * NaN = NaN
    a = Number(NumberType::Infinity);
    b = Number(NumberType::NaN);
    res = a * b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);

    // -Infinity * NaN = NaN
    a = Number(NumberType::NegInfinity);
    b = Number(NumberType::NaN);
    res = a * b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);

    // Infinity * -4.5 = -Infinity
    a = Number(NumberType::Infinity);
    b = Number(-4.5);
    res = a * b;
    EXPECT_EQ(res.get_type(), NumberType::NegInfinity);

    // 4.5 * Infinity = Infinity
    a = Number(4.5);
    b = Number(NumberType::Infinity);
    res = a * b;
    EXPECT_EQ(res.get_type(), NumberType::Infinity);

    // 4.5 * -Infinity = -Infinity
    a = Number(4.5);
    b = Number(NumberType::NegInfinity);
    res = a * b;
    EXPECT_EQ(res.get_type(), NumberType::NegInfinity);

    // Infinity * 4.5 = Infinity
    a = Number(NumberType::Infinity);
    b = Number(4.5);
    res = a * b;
    EXPECT_EQ(res.get_type(), NumberType::Infinity);

    // -Infinity * 4.5 = -Infinity
    a = Number(NumberType::NegInfinity);
    b = Number(4.5);
    res = a * b;
    EXPECT_EQ(res.get_type(), NumberType::NegInfinity);

    // -4.5 * Infinity = -Infinity
    a = Number(-4.5);
    b = Number(NumberType::Infinity);
    res = a * b;
    EXPECT_EQ(res.get_type(), NumberType::NegInfinity);

    // -Infinity * -4.5 = Infinity
    a = Number(NumberType::NegInfinity);
    b = Number(-4.5);
    res = a * b;
    EXPECT_EQ(res.get_type(), NumberType::Infinity);

    // -4.5 * -Infinity = Infinity
    a = Number(-4.5);
    b = Number(NumberType::NegInfinity);
    res = a * b;
    EXPECT_EQ(res.get_type(), NumberType::Infinity);

    // 0 * -Infinity = NaN
    a = Number(0);
    b = Number(NumberType::NegInfinity);
    res = a * b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);

    // 0 * Infinity = NaN
    a = Number(0);
    b = Number(NumberType::Infinity);
    res = a * b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);

    // -Infinity * 0 = NaN
    a = Number(NumberType::NegInfinity);
    b = Number(0);
    res = a * b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);

    // Infinity * 0 = NaN
    a = Number(NumberType::Infinity);
    b = Number(0);
    res = a * b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);

    // Infinity * -Infinity = -Infinity
    a = Number(NumberType::Infinity);
    b = Number(NumberType::NegInfinity);
    res = a * b;
    EXPECT_EQ(res.get_type(), NumberType::NegInfinity);

    // -Infinity * Infinity = -Infinity
    a = Number(NumberType::NegInfinity);
    b = Number(NumberType::Infinity);
    res = a * b;
    EXPECT_EQ(res.get_type(), NumberType::NegInfinity);

    // Infinity * Infinity = Infinity
    a = Number(NumberType::Infinity);
    b = Number(NumberType::Infinity);
    res = a * b;
    EXPECT_EQ(res.get_type(), NumberType::Infinity);

    // -Infinity * -Infinity = Infinity
    a = Number(NumberType::NegInfinity);
    b = Number(NumberType::NegInfinity);
    res = a * b;
    EXPECT_EQ(res.get_type(), NumberType::Infinity);
}

TEST(Number, division_test)
{
    Number a(0);
    Number b(0);
    Number res(0);

    // 6 / 3 = 2
    a = Number(6);
    b = Number(3);
    res = a / b;
    EXPECT_EQ(res.get_type(), NumberType::Number);
    EXPECT_EQ(res.get_value(), 2);

    // -6 / 3 = -2
    a = Number(-6);
    b = Number(3);
    res = a / b;
    EXPECT_EQ(res.get_type(), NumberType::Number);
    EXPECT_EQ(res.get_value(), -2);

    // 6 / -3 = -2
    a = Number(6);
    b = Number(-3);
    res = a / b;
    EXPECT_EQ(res.get_type(), NumberType::Number);
    EXPECT_EQ(res.get_value(), -2);

    // -6 / -3 = -2
    a = Number(-6);
    b = Number(-3);
    res = a / b;
    EXPECT_EQ(res.get_type(), NumberType::Number);
    EXPECT_EQ(res.get_value(), 2);

    // 3 / 0 = Infinity
    a = Number(3);
    b = Number(0);
    res = a / b;
    EXPECT_EQ(res.get_type(), NumberType::Infinity);

    // -3 / 0 = Infinity
    a = Number(-3);
    b = Number(0);
    res = a / b;
    EXPECT_EQ(res.get_type(), NumberType::NegInfinity);

    // 0 / 0 = NaN
    a = Number(0);
    b = Number(0);
    res = a / b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);

    // NaN / 2 = NaN
    a = Number(NumberType::NaN);
    b = Number(2);
    res = a / b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);

    // 2 / NaN = NaN
    a = Number(2);
    b = Number(NumberType::NaN);
    res = a / b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);

    // Infinity / NaN = NaN
    a = Number(NumberType::Infinity);
    b = Number(NumberType::NaN);
    res = a / b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);

    // -Infinity / NaN = NaN
    a = Number(NumberType::NegInfinity);
    b = Number(NumberType::NaN);
    res = a / b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);

    // NaN / Infinity = NaN
    a = Number(NumberType::NaN);
    b = Number(NumberType::Infinity);
    res = a / b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);

    // NaN / -Infinity = NaN
    a = Number(NumberType::NaN);
    b = Number(NumberType::NegInfinity);
    res = a / b;
    EXPECT_EQ(res.get_type(), NumberType::NaN);

    // Infinity / 1 = Infinity
    a = Number(NumberType::Infinity);
    b = Number(1);
    res = a / b;
    EXPECT_EQ(res.get_type(), NumberType::Infinity);

    // -Infinity / 1 = -Infinity
    a = Number(NumberType::NegInfinity);
    b = Number(1);
    res = a / b;
    EXPECT_EQ(res.get_type(), NumberType::NegInfinity);

    // Infinity / -1 = -Infinity
    a = Number(NumberType::Infinity);
    b = Number(-1);
    res = a / b;
    EXPECT_EQ(res.get_type(), NumberType::NegInfinity);

    // -Infinity / -1 = Infinity
    a = Number(NumberType::NegInfinity);
    b = Number(-1);
    res = a / b;
    EXPECT_EQ(res.get_type(), NumberType::Infinity);

    // 1 / Infinity = 0
    a = Number(1);
    b = Number(NumberType::Infinity);
    res = a / b;
    EXPECT_EQ(res.get_type(), NumberType::Number);
    EXPECT_EQ(res.get_value(), 0);

    // 1 / -Infinity = 0
    a = Number(1);
    b = Number(NumberType::NegInfinity);
    res = a / b;
    EXPECT_EQ(res.get_type(), NumberType::Number);
    EXPECT_EQ(res.get_value(), 0);
}