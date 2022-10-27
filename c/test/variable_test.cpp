#include <gtest/gtest.h>

#include "variable/variable.hpp"
#include "object/object.hpp"

TEST(Variable, Basic_test)
{
    Variable val1;
    Variable val2;

    EXPECT_EQ(val1.flag, Type::Undefined);
    EXPECT_EQ(val2.flag, Type::Undefined);

    val1.set_undefined();
    val2 = val1;
    EXPECT_EQ(val1.flag, Type::Undefined);
    EXPECT_EQ(val2.flag, Type::Undefined);

    val1.set_null();
    val2 = val1;
    EXPECT_EQ(val1.flag, Type::Null);
    EXPECT_EQ(val2.flag, Type::Null);

    val1.set_number(Number(13));
    val2 = val1;
    EXPECT_EQ(val1.flag, Type::Number);
    EXPECT_EQ(val2.flag, Type::Number);
    EXPECT_EQ(val1.number_field, Number(13));
    EXPECT_EQ(val2.number_field, Number(13));

    val1.set_boolean(true);
    val2 = val1;
    EXPECT_EQ(val1.flag, Type::Boolean);
    EXPECT_EQ(val2.flag, Type::Boolean);
    EXPECT_EQ(val1.boolean_field, true);
    EXPECT_EQ(val2.boolean_field, true);

    val1.set_string("foo");
    val2 = val1;
    EXPECT_EQ(val1.flag, Type::String);
    EXPECT_EQ(val2.flag, Type::String);
    EXPECT_EQ(val1.string_field, "foo");
    EXPECT_EQ(val2.string_field, "foo");

    val1.set_object(Object());
    val2 = val1;
    EXPECT_EQ(val1.flag, Type::Object);
    EXPECT_EQ(val2.flag, Type::Object);
    EXPECT_EQ(val1.object_field, Object{});
    EXPECT_EQ(val2.object_field, Object{});
}

TEST(Variable, to_boolean_test)
{
    Variable val;

    val.set_undefined();
    EXPECT_EQ(val.to_boolean(), false);

    val.set_null();
    EXPECT_EQ(val.to_boolean(), false);

    val.set_number(Number(1));
    EXPECT_EQ(val.to_boolean(), true);

    val.set_boolean(true);
    EXPECT_EQ(val.to_boolean(), true);

    val.set_boolean(false);
    EXPECT_EQ(val.to_boolean(), false);

    val.set_string("Hello world");
    EXPECT_EQ(val.to_boolean(), true);

    val.set_string("");
    EXPECT_EQ(val.to_boolean(), false);

    val.set_object(Object());
    EXPECT_EQ(val.to_boolean(), true);
}

TEST(Variable, to_number_test)
{
    Variable val;

    val.set_undefined();
    EXPECT_EQ(val.to_number(), Number(NumberType::NaN));

    val.set_null();
    EXPECT_EQ(val.to_number(), Number(0));

    val.set_number(Number(13));
    EXPECT_EQ(val.to_number(), Number(13));

    val.set_boolean(true);
    EXPECT_EQ(val.to_number(), Number(1));

    val.set_boolean(false);
    EXPECT_EQ(val.to_number(), Number(0));

    val.set_string("Hello world");
    EXPECT_EQ(val.to_number(), Number(NumberType::NaN));

    val.set_string("");
    EXPECT_EQ(val.to_number(), Number(NumberType::NaN));

    val.set_object(Object());
    EXPECT_EQ(val.to_number(), Number(NumberType::NaN));
}

TEST(Variable, to_string_test)
{
    Variable val;

    val.set_undefined();
    EXPECT_EQ(val.to_string(), "undefined");

    val.set_null();
    EXPECT_EQ(val.to_string(), "null");

    val.set_number(Number(NumberType::NaN));
    EXPECT_EQ(val.to_string(), "NaN");

    val.set_boolean(true);
    EXPECT_EQ(val.to_string(), "true");

    val.set_boolean(false);
    EXPECT_EQ(val.to_string(), "false");

    val.set_string("Hello world");
    EXPECT_EQ(val.to_string(), "Hello world");

    val.set_string("");
    EXPECT_EQ(val.to_string(), "");

    val.set_object(Object());
    EXPECT_EQ(val.to_string(), "{}");
}

TEST(Variable, arithmetic_test)
{
    Variable a;
    Variable b;
    Variable res;

    // "Hello " + "world" = "Hello world"
    a.set_string("Hello ");
    b.set_string("world");
    res = a + b;
    EXPECT_EQ(res.flag, Type::String);
    EXPECT_EQ(res.string_field, "Hello world");

    // 2 + " world" = "2.000000 world"
    a.set_number(2);
    b.set_string(" world");
    res = a + b;
    EXPECT_EQ(res.flag, Type::String);
    EXPECT_EQ(res.string_field, "2.000000 world");

    // "Hello " + 2 = "Hello 2.000000"
    a.set_string("Hello ");
    b.set_number(2);
    res = a + b;
    EXPECT_EQ(res.flag, Type::String);
    EXPECT_EQ(res.string_field, "Hello 2.000000");

    a.set_number(1);
    b.set_number(2);
    res = a + b;
    EXPECT_EQ(res.flag, Type::Number);
    EXPECT_EQ(res.number_field, Number(3));

    a.set_number(1);
    b.set_number(2);
    res = a - b;
    EXPECT_EQ(res.flag, Type::Number);
    EXPECT_EQ(res.number_field, Number(-1));

    a.set_number(1);
    b.set_number(2);
    res = a * b;
    EXPECT_EQ(res.flag, Type::Number);
    EXPECT_EQ(res.number_field, Number(2));

    a.set_number(2);
    b.set_number(1);
    res = a / b;
    EXPECT_EQ(res.flag, Type::Number);
    EXPECT_EQ(res.number_field, Number(2));
}

TEST(Variable, logical_not_test)
{
    
}