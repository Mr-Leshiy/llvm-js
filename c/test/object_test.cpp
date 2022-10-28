#include <gtest/gtest.h>

#include "object/object.hpp"
#include "variable/variable.hpp"

TEST(Object, Basic_test)
{
    Object object;

    Variable value;
    value.set_number(Number(12));
    object.add_property("name", &value);

    auto *prop = object.get_property("name");
    EXPECT_EQ(prop->get_flag(), value.get_flag());
    EXPECT_EQ(prop->get_number(), value.get_number());

    Variable key;
    key.set_string("name");
    prop = object.get_property(key);
    EXPECT_EQ(prop->get_flag(), value.get_flag());
    EXPECT_EQ(prop->get_number(), value.get_number());

    prop = object.get_property("age");
    EXPECT_EQ(prop->get_flag(), Type::Undefined);

    EXPECT_EQ(object.to_string(), "{name: 12.000000,}");

    object.remove_property("name");
    prop = object.get_property("name");
    EXPECT_EQ(prop->get_flag(), Type::Undefined);
}