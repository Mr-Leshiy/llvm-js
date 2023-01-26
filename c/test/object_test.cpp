#include <gtest/gtest.h>

#include "object/object.hpp"
#include "variable/variable.hpp"

TEST(Object, Basic_test)
{
    Object object;

    EXPECT_EQ(object.empty(), true);

    Variable &value = GarbageCollector<Variable>::get_instance().allocate()->set_number(Number(12));
    object.add_property("name", &value);
    EXPECT_EQ(object.empty(), false);

    EXPECT_EQ(object.to_string(), "{name: 12.000000,}");

    auto *prop = object.get_property("name");
    EXPECT_EQ(prop->get_flag(), value.get_flag());
    EXPECT_EQ(prop->get_number(), value.get_number());

    Variable &key = GarbageCollector<Variable>::get_instance().allocate()->set_string("name");
    prop = object.get_property(key.to_string());
    EXPECT_EQ(prop->get_flag(), value.get_flag());
    EXPECT_EQ(prop->get_number(), value.get_number());

    prop = object.get_property("age");
    EXPECT_EQ(prop->get_flag(), Type::Undefined);

    prop->set_null();
    prop = object.get_property("age");
    EXPECT_EQ(prop->get_flag(), Type::Null);

    object.remove_property("name");
    prop = object.get_property("name");
    EXPECT_EQ(prop->get_flag(), Type::Undefined);

    prop->set_null();
    prop = object.get_property("name");
    EXPECT_EQ(prop->get_flag(), Type::Null);
}