#include <gtest/gtest.h>

#include "array/array.hpp"
#include "variable/variable.hpp"

TEST(Array, Basic_test)
{
    Array array;
    Variable *variable;

    EXPECT_EQ(array.len(), 0);
    EXPECT_EQ(array.to_string(), "[]");

    Variable &var1 = GarbageCollector<Variable>::get_instance().allocate()->set_number(Number(2));
    Variable &var2 = GarbageCollector<Variable>::get_instance().allocate()->set_string("name");
    Variable &var3 = GarbageCollector<Variable>::get_instance().allocate()->set_boolean(true);

    array = Array({&var1, &var2, &var3});

    EXPECT_EQ(array.len(), 3);
    EXPECT_EQ(array.to_string(), "[2.000000,name,true,]");

    variable = array.get(0, false);
    EXPECT_EQ(variable->get_flag(), Type::Number);
    EXPECT_EQ(variable->get_number(), 2);

    variable = array.get(1, false);
    EXPECT_EQ(variable->get_flag(), Type::String);
    EXPECT_EQ(variable->get_string(), "name");

    variable = array.get(2, false);
    EXPECT_EQ(variable->get_flag(), Type::Boolean);
    EXPECT_EQ(variable->get_boolean(), true);

    variable = array.get(3, false);
    EXPECT_EQ(variable->get_flag(), Type::Undefined);

    variable = array.pop();
    EXPECT_EQ(variable->get_flag(), Type::Boolean);
    EXPECT_EQ(variable->get_boolean(), true);
    EXPECT_EQ(array.len(), 2);

    variable = array.pop();
    EXPECT_EQ(variable->get_flag(), Type::String);
    EXPECT_EQ(variable->get_string(), "name");
    EXPECT_EQ(array.len(), 1);

    variable = array.pop();
    EXPECT_EQ(variable->get_flag(), Type::Number);
    EXPECT_EQ(variable->get_number(), 2);
    EXPECT_EQ(array.len(), 0);

    variable = array.pop();
    EXPECT_EQ(variable->get_flag(), Type::Undefined);
    EXPECT_EQ(array.len(), 0);

    array.push(var1);
    array.push(var2);
    array.push(var3);

    EXPECT_EQ(array.len(), 3);

    variable = array.get(0, false);
    EXPECT_EQ(variable->get_flag(), Type::Number);
    EXPECT_EQ(variable->get_number(), 2);

    variable = array.get(1, false);
    EXPECT_EQ(variable->get_flag(), Type::String);
    EXPECT_EQ(variable->get_string(), "name");

    variable = array.get(2, false);
    EXPECT_EQ(variable->get_flag(), Type::Boolean);
    EXPECT_EQ(variable->get_boolean(), true);

    array.put(var1, 4);

    EXPECT_EQ(array.len(), 5);

    variable = array.get(0, false);
    EXPECT_EQ(variable->get_flag(), Type::Number);
    EXPECT_EQ(variable->get_number(), 2);

    variable = array.get(1, false);
    EXPECT_EQ(variable->get_flag(), Type::String);
    EXPECT_EQ(variable->get_string(), "name");

    variable = array.get(2, false);
    EXPECT_EQ(variable->get_flag(), Type::Boolean);
    EXPECT_EQ(variable->get_boolean(), true);

    variable = array.get(3, false);
    EXPECT_EQ(variable->get_flag(), Type::Undefined);

    variable = array.get(4, false);
    EXPECT_EQ(variable->get_flag(), Type::Number);
    EXPECT_EQ(variable->get_number(), 2);

    variable = array.get(6, true);
    EXPECT_EQ(variable->get_flag(), Type::Undefined);

    EXPECT_EQ(array.len(), 7);

    variable->set_null();
    variable = array.get(5, false);
    EXPECT_EQ(variable->get_flag(), Type::Undefined);
    variable = array.get(6, false);
    EXPECT_EQ(variable->get_flag(), Type::Null);
}