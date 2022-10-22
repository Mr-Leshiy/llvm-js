#include <gtest/gtest.h>

#include "object.hpp"
#include "variable.hpp"

TEST(Object, Basic_test)
{
    Object object;

    auto *value = allocate();
    set_number(value, 12);
    object.add_property("name", value);

    auto *prop = object.get_property("name");
    EXPECT_EQ(prop->flag, value->flag);
    EXPECT_EQ(prop->number_field, value->number_field);

    Variable key;
    set_string(&key, "name");
    prop = object.get_property(key);
    EXPECT_EQ(prop->flag, value->flag);
    EXPECT_EQ(prop->number_field, value->number_field);

    prop = object.get_property("age");
    EXPECT_EQ(prop->flag, Type::Undefined);

    EXPECT_EQ(object.to_string(), "{name: 12.000000,}");

    object.remove_property("name");
    prop = object.get_property("name");
    EXPECT_EQ(prop->flag, Type::Undefined);
}