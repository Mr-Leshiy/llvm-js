#include <gtest/gtest.h>

#include "object.hpp"
#include "variable.hpp"

TEST(Object, Basic_test)
{
    Object object;

    auto *value = allocate();
    set_number(value, 12);
    add_property(object, "name", value);

    auto *prop = get_property(object, "name");
    EXPECT_EQ(prop->flag, value->flag);
    EXPECT_EQ(prop->number_field, value->number_field);

    prop = get_property(object, "age");
    EXPECT_EQ(prop->flag, Type::Undefined);

    EXPECT_EQ(object.to_string(), "{name: 12.000000}");

    remove_property(object, "name");
    prop = get_property(object, "name");
    EXPECT_EQ(prop->flag, Type::Undefined);
}