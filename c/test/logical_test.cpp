#include <gtest/gtest.h>
#include <string.h>

extern "C" {
#include "logical.h"
}

TEST(Logical, logical_not_test)
{
    VariableType* res;
    VariableType* val = allocate();

    set_boolean(val, true);
    res = logical_not(val);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, false);

    set_boolean(val, false);
    res = logical_not(val);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, true);

    set_number(val, 2.5);
    res = logical_not(val);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, false);

    set_number(val, 0);
    res = logical_not(val);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, true);

    set_string(val, "Hello world");
    res = logical_not(val);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, false);

    set_string(val, "");
    res = logical_not(val);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, true);

    // TODO add more test cases
}

TEST(Logical, logical_and_test)
{
    VariableType* res;
    VariableType* val1 = allocate();
    VariableType* val2 = allocate();

    set_boolean(val2, true);
    set_boolean(val1, true);
    res = logical_and(val1, val2);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, true);

    set_boolean(val1, false);
    set_boolean(val2, true);
    res = logical_and(val1, val2);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, false);

    set_boolean(val2, true);
    set_boolean(val1, false);
    res = logical_and(val1, val2);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, false);

    set_boolean(val1, false);
    set_boolean(val2, false);
    res = logical_and(val1, val2);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, false);
}

TEST(Logical, logical_or_test)
{
    VariableType* res;
    VariableType* val1 = allocate();
    VariableType* val2 = allocate();

    set_boolean(val2, true);
    set_boolean(val1, true);
    res = logical_or(val1, val2);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, true);

    set_boolean(val1, false);
    set_boolean(val2, true);
    res = logical_or(val1, val2);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, true);

    set_boolean(val2, true);
    set_boolean(val1, false);
    res = logical_or(val1, val2);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, true);

    set_boolean(val1, false);
    set_boolean(val2, false);
    res = logical_or(val1, val2);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, false);

    // TODO add more test cases
}

TEST(Logical, logical_seq_test)
{
    VariableType* res;
    VariableType* val1 = allocate();
    VariableType* val2 = allocate();

    set_number(val2, 3);
    set_number(val1, 3);
    res = logical_seq(val1, val2);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, true);

    set_number(val2, 3);
    set_number(val1, 4);
    res = logical_seq(val1, val2);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, false);

    set_boolean(val2, true);
    set_boolean(val1, false);
    res = logical_seq(val1, val2);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, false);

    set_boolean(val1, false);
    set_boolean(val2, false);
    res = logical_seq(val1, val2);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, true);

    set_string(val1, "hello");
    set_string(val2, "hello");
    res = logical_seq(val1, val2);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, true);

    set_string(val1, "hello");
    set_string(val2, "hell");
    res = logical_seq(val1, val2);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, false);

    set_number(val2, 3);
    set_boolean(val1, true);
    res = logical_seq(val1, val2);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, false);

    set_number(val2, 3);
    set_string(val1, "hello");
    res = logical_seq(val1, val2);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, false);

    set_boolean(val2, true);
    set_string(val1, "hello");
    res = logical_seq(val1, val2);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, false);
}

TEST(Logical, logical_sne_test)
{
    VariableType* res;
    VariableType* val1 = allocate();
    VariableType* val2 = allocate();

    set_number(val2, 3);
    set_number(val1, 3);
    res = logical_sne(val1, val2);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, false);

    set_number(val2, 3);
    set_number(val1, 4);
    res = logical_sne(val1, val2);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, true);

    set_boolean(val2, true);
    set_boolean(val1, false);
    res = logical_sne(val1, val2);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, true);

    set_boolean(val1, false);
    set_boolean(val2, false);
    res = logical_sne(val1, val2);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, false);

    set_string(val1, "hello");
    set_string(val2, "hello");
    res = logical_sne(val1, val2);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, false);

    set_string(val1, "hello");
    set_string(val2, "hell");
    res = logical_sne(val1, val2);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, true);

    set_number(val2, 3);
    set_boolean(val1, true);
    res = logical_sne(val1, val2);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, true);

    set_number(val2, 3);
    set_string(val1, "hello");
    res = logical_sne(val1, val2);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, true);

    set_boolean(val2, true);
    set_string(val1, "hello");
    res = logical_sne(val1, val2);
    EXPECT_EQ(res->flag, Boolean);
    EXPECT_EQ(res->boolean_field, true);
}