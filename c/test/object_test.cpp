#include <gtest/gtest.h>

#include "object/object.hpp"
#include "variable/variable.hpp"

TEST(Object, Basic_test)
{
    Object object;

    Variable value = test::VariableTest().set_number(Number(12));
    object.add_property("name", &value);

    EXPECT_EQ(object.to_string(), "{name: 12.000000,}");

    auto *prop = object.get_property("name", false);
    EXPECT_EQ(prop->get_flag(), value.get_flag());
    EXPECT_EQ(prop->get_number(), value.get_number());

    Variable key = test::VariableTest().set_string("name");
    prop = object.get_property(key.to_string(), false);
    EXPECT_EQ(prop->get_flag(), value.get_flag());
    EXPECT_EQ(prop->get_number(), value.get_number());

    prop = object.get_property("age", true);
    EXPECT_EQ(prop->get_flag(), Type::Undefined);

    prop->set_null();
    prop = object.get_property("age", true);
    EXPECT_EQ(prop->get_flag(), Type::Null);

    object.remove_property("name");
    prop = object.get_property("name", false);
    EXPECT_EQ(prop->get_flag(), Type::Undefined);

    prop->set_null();
    prop = object.get_property("name", true);
    EXPECT_EQ(prop->get_flag(), Type::Undefined);
}