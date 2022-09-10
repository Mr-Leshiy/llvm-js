#include <gtest/gtest.h>
#include <string.h>

extern "C" {
#include "variable.h"
}

TEST(VariableType, Basic_test)
{
    VariableType* val1 = allocate();
    VariableType* val2 = allocate();

    EXPECT_NE(val1, nullptr);
    EXPECT_NE(val2, nullptr);
    EXPECT_EQ(val1->flag, 0);
    EXPECT_EQ(val2->flag, 0);

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
    VariableType* res;
    VariableType* val = allocate();

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