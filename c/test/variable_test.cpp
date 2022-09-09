#include <gtest/gtest.h>
#include <string.h>

extern "C" {
#include "variable.h"
}

TEST(VariableType, Basic_test)
{
    VariableType* var1 = allocate();
    VariableType* var2 = allocate();

    EXPECT_NE(var1, nullptr);
    EXPECT_NE(var2, nullptr);
    EXPECT_EQ(var1->flag, 0);
    EXPECT_EQ(var2->flag, 0);

    set_number(var1, 2.0);
    set_variable(var2, var1);
    EXPECT_EQ(var1->flag, Number);
    EXPECT_EQ(var2->flag, Number);
    EXPECT_EQ(var1->number_field, 2.0);
    EXPECT_EQ(var2->number_field, 2.0);

    set_boolean(var1, true);
    set_variable(var2, var1);
    EXPECT_EQ(var1->flag, Boolean);
    EXPECT_EQ(var2->flag, Boolean);
    EXPECT_EQ(var1->boolean_field, true);
    EXPECT_EQ(var2->boolean_field, true);

    set_string(var1, "foo");
    set_variable(var2, var1);
    EXPECT_EQ(var1->flag, String);
    EXPECT_EQ(var2->flag, String);
    EXPECT_EQ(strcmp(var1->string_field, "foo"), 0);
    EXPECT_EQ(strcmp(var2->string_field, "foo"), 0);
}