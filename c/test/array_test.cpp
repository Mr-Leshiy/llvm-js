#include <gtest/gtest.h>

#include "array/array.hpp"
#include "variable/variable.hpp"

TEST(Array, Basic_test)
{
    Array array;
    Variable *variable;

    EXPECT_EQ(array.len(), 0);
    EXPECT_EQ(array.to_string(), "[]");

    Variable *var1 = allocate();
    set_number(var1, 2);
    Variable *var2 = allocate();
    set_string(var2, "name");
    Variable *var3 = allocate();
    set_boolean(var3, true);

    array = Array({var1, var2, var3});

    EXPECT_EQ(array.len(), 3);
    EXPECT_EQ(array.to_string(), "[2.000000,name,true,]");

    variable = array.get(0);
    EXPECT_EQ(variable->flag, Type::Number);
    EXPECT_EQ(variable->number_field, 2);

    variable = array.get(1);
    EXPECT_EQ(variable->flag, Type::String);
    EXPECT_EQ(variable->string_field, "name");

    variable = array.get(2);
    EXPECT_EQ(variable->flag, Type::Boolean);
    EXPECT_EQ(variable->boolean_field, true);

    variable = array.get(3);
    EXPECT_EQ(variable->flag, Type::Undefined);

    variable = array.get(-1);
    EXPECT_EQ(variable->flag, Type::Undefined);

    variable = array.pop();
    EXPECT_EQ(variable->flag, Type::Boolean);
    EXPECT_EQ(variable->boolean_field, true);
    EXPECT_EQ(array.len(), 2);

    variable = array.pop();
    EXPECT_EQ(variable->flag, Type::String);
    EXPECT_EQ(variable->string_field, "name");
    EXPECT_EQ(array.len(), 1);

    variable = array.pop();
    EXPECT_EQ(variable->flag, Type::Number);
    EXPECT_EQ(variable->number_field, 2);
    EXPECT_EQ(array.len(), 0);

    variable = array.pop();
    EXPECT_EQ(variable->flag, Type::Undefined);
    EXPECT_EQ(array.len(), 0);
}