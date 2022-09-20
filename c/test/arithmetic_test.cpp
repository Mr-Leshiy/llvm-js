#include <gtest/gtest.h>
#include <string.h>

#include "arithmetic.hpp"
#include "variable.hpp"

TEST(Logical, arithmetic_addition_test)
{
    Variable *res;
    Variable *val1 = allocate();
    Variable *val2 = allocate();

    set_number(val1, 2);
    set_number(val2, 3);
    res = arithmetic_addition(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, 5);

    set_number(val1, -2);
    set_number(val2, 3);
    res = arithmetic_addition(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, 1);

    set_number(val1, -2.5);
    set_number(val2, -4.5);
    res = arithmetic_addition(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, -7);

    set_nan(val1);
    set_number(val2, -4.5);
    res = arithmetic_addition(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    set_number(val1, -4.5);
    set_nan(val2);
    res = arithmetic_addition(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);
}

TEST(Logical, arithmetic_substraction_test)
{
    Variable *res;
    Variable *val1 = allocate();
    Variable *val2 = allocate();

    set_number(val1, 2);
    set_number(val2, 3);
    res = arithmetic_substraction(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, -1);

    set_number(val1, -2);
    set_number(val2, 3);
    res = arithmetic_substraction(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, -5);

    set_number(val1, -2.5);
    set_number(val2, -4.5);
    res = arithmetic_substraction(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, 2);

    set_nan(val1);
    set_number(val2, -4.5);
    res = arithmetic_substraction(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    set_number(val1, -4.5);
    set_nan(val2);
    res = arithmetic_substraction(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);
}

TEST(Logical, arithmetic_multiplication_test)
{
    Variable *res;
    Variable *val1 = allocate();
    Variable *val2 = allocate();

    set_number(val1, 2);
    set_number(val2, 3);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, 6);

    set_number(val1, -2);
    set_number(val2, 3);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, -6);

    set_number(val1, -2.5);
    set_number(val2, -4.5);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, 11.25);

    set_nan(val1);
    set_number(val2, -4.5);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);

    set_number(val1, -4.5);
    set_nan(val2);
    res = arithmetic_multiplication(val1, val2);
    EXPECT_EQ(res->flag, Type::NaN);
}