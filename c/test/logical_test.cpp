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
}
