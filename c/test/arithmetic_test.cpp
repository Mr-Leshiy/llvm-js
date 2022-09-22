#include <gtest/gtest.h>
#include <string.h>

#include "arithmetic.hpp"
#include "variable.hpp"

TEST(Logical, arithmetic_addition_test)
{
    Variable *res;
    Variable *val1 = allocate();
    Variable *val2 = allocate();

    // 2 + 3 = 5
    set_number(val1, 2);
    set_number(val2, 3);
    res = arithmetic_addition(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, 5);

    // 2 + 0 = 2
    set_number(val1, 2);
    set_number(val2, 0);
    res = arithmetic_addition(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, 2);

    // 0 + 2 = 2
    set_number(val1, 0);
    set_number(val2, 2);
    res = arithmetic_addition(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, 2);

    // -2 + 3 = 1
    set_number(val1, -2);
    set_number(val2, 3);
    res = arithmetic_addition(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, 1);

    // -2.5 + -4.5 = -7
    set_number(val1, -2.5);
    set_number(val2, -4.5);
    res = arithmetic_addition(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, -7);

    // NaN + -4.5 = NaN
    set_nan(val1);
    set_number(val2, -4.5);
    res = arithmetic_addition(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    // -4.5 + NaN = NaN
    set_number(val1, -4.5);
    set_nan(val2);
    res = arithmetic_addition(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    // Infinity + NaN = NaN
    set_infinity(val1);
    set_nan(val2);
    res = arithmetic_addition(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    // -Infinity + NaN = NaN
    set_neginfinity(val1);
    set_nan(val2);
    res = arithmetic_addition(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    // Infinity + -4.5 = Infinity
    set_infinity(val1);
    set_number(val2, -4.5);
    res = arithmetic_addition(val1, val2);
    EXPECT_EQ(res->flag, Type::Infinity);

    // -Infinity + -4.5 = -Infinity
    set_neginfinity(val1);
    set_number(val2, -4.5);
    res = arithmetic_addition(val1, val2);
    EXPECT_EQ(res->flag, Type::NegInfinity);

    // Infinity + -Infinity = NaN
    set_infinity(val1);
    set_neginfinity(val2);
    res = arithmetic_addition(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    // -Infinity + Infinity = NaN
    set_neginfinity(val1);
    set_infinity(val2);
    res = arithmetic_addition(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    // Infinity + Infinity = Infinity
    set_infinity(val1);
    set_infinity(val2);
    res = arithmetic_addition(val1, val2);
    EXPECT_EQ(res->flag, Type::Infinity);

    // -Infinity + -Infinity = -Infinity
    set_neginfinity(val1);
    set_neginfinity(val2);
    res = arithmetic_addition(val1, val2);
    EXPECT_EQ(res->flag, Type::NegInfinity);
}

TEST(Logical, arithmetic_substraction_test)
{
    Variable *res;
    Variable *val1 = allocate();
    Variable *val2 = allocate();

    // 2 - 3 = -1
    set_number(val1, 2);
    set_number(val2, 3);
    res = arithmetic_substraction(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, -1);

    // 2 - 0 = 2
    set_number(val1, 2);
    set_number(val2, 0);
    res = arithmetic_substraction(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, 2);

    // 0 - 2 = -2
    set_number(val1, 0);
    set_number(val2, 2);
    res = arithmetic_substraction(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, -2);

    // -2 - 3 = -5
    set_number(val1, -2);
    set_number(val2, 3);
    res = arithmetic_substraction(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, -5);

    // -2.5 - -4.5 = 2
    set_number(val1, -2.5);
    set_number(val2, -4.5);
    res = arithmetic_substraction(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, 2);

    // NaN - -4.5 = NaN
    set_nan(val1);
    set_number(val2, -4.5);
    res = arithmetic_substraction(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    // -4.5 - NaN = NaN
    set_number(val1, -4.5);
    set_nan(val2);
    res = arithmetic_substraction(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    // Infinity - NaN = NaN
    set_infinity(val1);
    set_nan(val2);
    res = arithmetic_substraction(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    // -Infinity - NaN = NaN
    set_neginfinity(val1);
    set_nan(val2);
    res = arithmetic_substraction(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    // Infinity - -4.5 = Infinity
    set_infinity(val1);
    set_number(val2, -4.5);
    res = arithmetic_substraction(val1, val2);
    EXPECT_EQ(res->flag, Type::Infinity);

    // -Infinity - -4.5 = -Infinity
    set_neginfinity(val1);
    set_number(val2, -4.5);
    res = arithmetic_substraction(val1, val2);
    EXPECT_EQ(res->flag, Type::NegInfinity);

    // Infinity - -Infinity = Infinity
    set_infinity(val1);
    set_neginfinity(val2);
    res = arithmetic_substraction(val1, val2);
    EXPECT_EQ(res->flag, Type::Infinity);

    // -Infinity - Infinity = -Infinity
    set_neginfinity(val1);
    set_infinity(val2);
    res = arithmetic_substraction(val1, val2);
    EXPECT_EQ(res->flag, Type::NegInfinity);

    // Infinity - Infinity = NaN
    set_infinity(val1);
    set_infinity(val2);
    res = arithmetic_substraction(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    // -Infinity - -Infinity = NaN
    set_neginfinity(val1);
    set_neginfinity(val2);
    res = arithmetic_substraction(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);
}

TEST(Logical, arithmetic_multiplication_test)
{
    Variable *res;
    Variable *val1 = allocate();
    Variable *val2 = allocate();

    // 2 * 3 = 6
    set_number(val1, 2);
    set_number(val2, 3);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, 6);

    // 2 * 0 = 0
    set_number(val1, 2);
    set_number(val2, 0);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, 0);

    // 0 * 2 = 0
    set_number(val1, 0);
    set_number(val2, 2);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, 0);

    // -2 * 3 = -6
    set_number(val1, -2);
    set_number(val2, 3);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, -6);

    // -2.5 * -4.5 = 11.25
    set_number(val1, -2.5);
    set_number(val2, -4.5);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, 11.25);

    // NaN * -4.5 = NaN
    set_nan(val1);
    set_number(val2, -4.5);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    // -4.5 * NaN = NaN
    set_number(val1, -4.5);
    set_nan(val2);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    // Infinity * NaN = NaN
    set_infinity(val1);
    set_nan(val2);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    // -Infinity * NaN = NaN
    set_neginfinity(val1);
    set_nan(val2);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    // Infinity * -4.5 = -Infinity
    set_infinity(val1);
    set_number(val2, -4.5);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::NegInfinity);

    // 4.5 * Infinity = Infinity
    set_number(val1, 4.5);
    set_infinity(val2);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::Infinity);

    // 4.5 * -Infinity = -Infinity
    set_number(val1, 4.5);
    set_neginfinity(val2);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::NegInfinity);

    // Infinity * 4.5 = Infinity
    set_infinity(val1);
    set_number(val2, 4.5);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::Infinity);

    // -Infinity * 4.5 = -Infinity
    set_neginfinity(val1);
    set_number(val2, 4.5);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::NegInfinity);

    // -4.5 * Infinity = -Infinity
    set_number(val1, -4.5);
    set_infinity(val2);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::NegInfinity);

    // -Infinity * -4.5 = Infinity
    set_neginfinity(val1);
    set_number(val2, -4.5);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::Infinity);

    // -4.5 * -Infinity = Infinity
    set_number(val1, -4.5);
    set_neginfinity(val2);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::Infinity);

    // 0 * -Infinity = NaN
    set_number(val1, 0);
    set_neginfinity(val2);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    // 0 * Infinity = NaN
    set_number(val1, 0);
    set_infinity(val2);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    // -Infinity * 0 = NaN
    set_neginfinity(val1);
    set_number(val2, 0);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    // Infinity * 0 = NaN
    set_infinity(val1);
    set_number(val2, 0);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    // Infinity * -Infinity = -Infinity
    set_infinity(val1);
    set_neginfinity(val2);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::NegInfinity);

    // -Infinity * Infinity = -Infinity
    set_neginfinity(val1);
    set_infinity(val2);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::NegInfinity);

    // Infinity * Infinity = Infinity
    set_infinity(val1);
    set_infinity(val2);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::Infinity);

    // -Infinity * -Infinity = Infinity
    set_neginfinity(val1);
    set_neginfinity(val2);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::Infinity);
}

TEST(Logical, arithmetic_division_test)
{
    Variable *res;
    Variable *val1 = allocate();
    Variable *val2 = allocate();

    // 6 / 3 = 2
    set_number(val1, 6);
    set_number(val2, 3);
    res = arithmetic_division(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, 2);

    // -6 / 3 = -2
    set_number(val1, -6);
    set_number(val2, 3);
    res = arithmetic_division(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, -2);

    // 6 / -3 = -2
    set_number(val1, 6);
    set_number(val2, -3);
    res = arithmetic_division(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, -2);

    // -6 / -3 = -2
    set_number(val1, -6);
    set_number(val2, -3);
    res = arithmetic_division(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, 2);

    // 3 / 0 = Infinity
    set_number(val1, 3);
    set_number(val2, 0);
    res = arithmetic_division(val1, val2);
    EXPECT_EQ(res->flag, Type::Infinity);

    // -3 / 0 = -Infinity
    set_number(val1, -3);
    set_number(val2, 0);
    res = arithmetic_division(val1, val2);
    EXPECT_EQ(res->flag, Type::NegInfinity);

    // 0 / 0 = NaN
    set_number(val1, 0);
    set_number(val2, 0);
    res = arithmetic_division(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    // NaN / 2 = NaN
    set_nan(val1);
    set_number(val2, 2);
    res = arithmetic_division(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    // 2 / NaN = NaN
    set_number(val1, 2);
    set_nan(val2);
    res = arithmetic_division(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    // Infinity / NaN = NaN
    set_infinity(val1);
    set_nan(val2);
    res = arithmetic_division(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    // -Infinity / NaN = NaN
    set_neginfinity(val1);
    set_nan(val2);
    res = arithmetic_division(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    // NaN / Infinity = NaN
    set_nan(val1);
    set_infinity(val2);
    res = arithmetic_division(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    // NaN / -Infinity = NaN
    set_nan(val1);
    set_neginfinity(val2);
    res = arithmetic_division(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    // Infinity / 1 = Infinity
    set_infinity(val1);
    set_number(val2, 1);
    res = arithmetic_division(val1, val2);
    EXPECT_EQ(res->flag, Type::Infinity);

    // -Infinity / 1 = -Infinity
    set_neginfinity(val1);
    set_number(val2, 1);
    res = arithmetic_division(val1, val2);
    EXPECT_EQ(res->flag, Type::NegInfinity);

    // Infinity / -1 = -Infinity
    set_infinity(val1);
    set_number(val2, -1);
    res = arithmetic_division(val1, val2);
    EXPECT_EQ(res->flag, Type::NegInfinity);

    // -Infinity / -1 = Infinity
    set_neginfinity(val1);
    set_number(val2, -1);
    res = arithmetic_division(val1, val2);
    EXPECT_EQ(res->flag, Type::Infinity);

    // 1 / Infinity = 0
    set_number(val1, 1);
    set_infinity(val2);
    res = arithmetic_division(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, 0);

    // 1 / -Infinity = 0
    set_number(val1, 1);
    set_neginfinity(val2);
    res = arithmetic_division(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, 0);
}