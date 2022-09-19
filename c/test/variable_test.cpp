#include <gtest/gtest.h>
#include <string.h>

extern "C"
{
#include "variable.h"
}

TEST(Variable, Basic_test)
{
    Variable *val1 = allocate();
    Variable *val2 = allocate();

    EXPECT_NE(val1, nullptr);
    EXPECT_NE(val2, nullptr);
    EXPECT_EQ(val1->flag, Undefined);
    EXPECT_EQ(val2->flag, Undefined);

    set_undefined(val1);
    set_variable(val2, val1);
    EXPECT_EQ(val1->flag, Undefined);
    EXPECT_EQ(val2->flag, Undefined);

    set_null(val1);
    set_variable(val2, val1);
    EXPECT_EQ(val1->flag, Null);
    EXPECT_EQ(val2->flag, Null);

    set_nan(val1);
    set_variable(val2, val1);
    EXPECT_EQ(val1->flag, NaN);
    EXPECT_EQ(val2->flag, NaN);

    set_number(val1, 2.0);
    set_variable(val2, val1);
    EXPECT_EQ(val1->flag, Number);
    EXPECT_EQ(val2->flag, Number);
    EXPECT_EQ(val1->number_field, 2.0);
    EXPECT_EQ(val2->number_field, 2.0);

    set_boolean(val1, true);
    set_variable(val2, val1);
    EXPECT_EQ(val1->flag, Boolean);
    EXPECT_EQ(val2->flag, Boolean);
    EXPECT_EQ(val1->boolean_field, true);
    EXPECT_EQ(val2->boolean_field, true);

    set_string(val1, "foo");
    set_variable(val2, val1);
    EXPECT_EQ(val1->flag, String);
    EXPECT_EQ(val2->flag, String);
    EXPECT_EQ(strcmp(val1->string_field, "foo"), 0);
    EXPECT_EQ(strcmp(val2->string_field, "foo"), 0);
}

TEST(VariableTest, convert_to_boolean_test)
{
    Variable *res;
    Variable *val = allocate();

    set_undefined(val);
    res = convert_to_boolean(val);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, false);

    set_null(val);
    res = convert_to_boolean(val);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, false);

    set_nan(val);
    res = convert_to_boolean(val);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, false);

    set_boolean(val, true);
    res = convert_to_boolean(val);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, true);

    set_boolean(val, false);
    res = convert_to_boolean(val);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, false);

    set_number(val, 2.5);
    res = convert_to_boolean(val);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, true);

    set_number(val, 0);
    res = convert_to_boolean(val);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, false);

    set_string(val, "Hello world");
    res = convert_to_boolean(val);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, true);

    set_string(val, "");
    res = convert_to_boolean(val);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, false);
}

TEST(VariableTest, convert_to_number_test)
{
    Variable *res;
    Variable *val = allocate();

    set_undefined(val);
    res = convert_to_number(val);
    EXPECT_EQ(res->flag, NaN);

    set_null(val);
    res = convert_to_number(val);
    EXPECT_EQ(res->flag, Number);
    EXPECT_EQ(res->number_field, 0);

    set_nan(val);
    res = convert_to_number(val);
    EXPECT_EQ(res->flag, NaN);

    set_boolean(val, true);
    res = convert_to_number(val);
    EXPECT_EQ(res->flag, Number);
    EXPECT_EQ(res->number_field, 1);

    set_boolean(val, false);
    res = convert_to_number(val);
    EXPECT_EQ(res->flag, Number);
    EXPECT_EQ(res->number_field, 0);

    set_number(val, 2.5);
    res = convert_to_number(val);
    EXPECT_EQ(res->flag, Number);
    EXPECT_EQ(res->number_field, 2.5);

    set_number(val, 0);
    res = convert_to_number(val);
    EXPECT_EQ(res->flag, Number);
    EXPECT_EQ(res->number_field, 0);

    set_string(val, "Hello world");
    res = convert_to_number(val);
    EXPECT_EQ(res->flag, NaN);

    set_string(val, "");
    res = convert_to_number(val);
    EXPECT_EQ(res->flag, NaN);
    // EXPECT_EQ(res->boolean_field, false);
}