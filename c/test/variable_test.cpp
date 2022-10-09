#include <gtest/gtest.h>

#include "variable.hpp"
#include "object.hpp"

TEST(Variable, Basic_test)
{
    Variable *val1 = variable_allocate();
    Variable *val2 = variable_allocate();

    EXPECT_NE(val1, nullptr);
    EXPECT_NE(val2, nullptr);
    EXPECT_EQ(val1->flag, Type::Undefined);
    EXPECT_EQ(val2->flag, Type::Undefined);

    set_undefined(val1);
    set_variable(val2, val1);
    EXPECT_EQ(val1->flag, Type::Undefined);
    EXPECT_EQ(val2->flag, Type::Undefined);

    set_null(val1);
    set_variable(val2, val1);
    EXPECT_EQ(val1->flag, Type::Null);
    EXPECT_EQ(val2->flag, Type::Null);

    set_nan(val1);
    set_variable(val2, val1);
    EXPECT_EQ(val1->flag, Type::NaN);
    EXPECT_EQ(val2->flag, Type::NaN);

    set_infinity(val1);
    set_variable(val2, val1);
    EXPECT_EQ(val1->flag, Type::Infinity);
    EXPECT_EQ(val2->flag, Type::Infinity);

    set_neginfinity(val1);
    set_variable(val2, val1);
    EXPECT_EQ(val1->flag, Type::NegInfinity);
    EXPECT_EQ(val2->flag, Type::NegInfinity);

    set_number(val1, 2.0);
    set_variable(val2, val1);
    EXPECT_EQ(val1->flag, Type::Number);
    EXPECT_EQ(val2->flag, Type::Number);
    EXPECT_EQ(val1->number_field, 2.0);
    EXPECT_EQ(val2->number_field, 2.0);

    set_boolean(val1, true);
    set_variable(val2, val1);
    EXPECT_EQ(val1->flag, Type::Boolean);
    EXPECT_EQ(val2->flag, Type::Boolean);
    EXPECT_EQ(val1->boolean_field, true);
    EXPECT_EQ(val2->boolean_field, true);

    set_string(val1, "foo");
    set_variable(val2, val1);
    EXPECT_EQ(val1->flag, Type::String);
    EXPECT_EQ(val2->flag, Type::String);
    EXPECT_EQ(val1->string_field, "foo");
    EXPECT_EQ(val2->string_field, "foo");

    Object object;
    set_object(val1, object);
    set_variable(val2, val1);
    EXPECT_EQ(val1->flag, Type::Object);
    EXPECT_EQ(val2->flag, Type::Object);
    EXPECT_EQ(val1->object_field, object);
    EXPECT_EQ(val2->object_field, object);
}

TEST(VariableTest, convert_to_boolean_test)
{
    Variable *res;
    Variable *val = variable_allocate();

    set_undefined(val);
    res = convert_to_boolean(val);
    EXPECT_EQ(res->flag, Type::Boolean);
    EXPECT_EQ(res->boolean_field, false);

    set_null(val);
    res = convert_to_boolean(val);
    EXPECT_EQ(res->flag, Type::Boolean);
    EXPECT_EQ(res->boolean_field, false);

    set_nan(val);
    res = convert_to_boolean(val);
    EXPECT_EQ(res->flag, Type::Boolean);
    EXPECT_EQ(res->boolean_field, false);

    set_infinity(val);
    res = convert_to_boolean(val);
    EXPECT_EQ(res->flag, Type::Boolean);
    EXPECT_EQ(res->boolean_field, true);

    set_neginfinity(val);
    res = convert_to_boolean(val);
    EXPECT_EQ(res->flag, Type::Boolean);
    EXPECT_EQ(res->boolean_field, true);

    set_boolean(val, true);
    res = convert_to_boolean(val);
    EXPECT_EQ(res->flag, Type::Boolean);
    EXPECT_EQ(res->boolean_field, true);

    set_boolean(val, false);
    res = convert_to_boolean(val);
    EXPECT_EQ(res->flag, Type::Boolean);
    EXPECT_EQ(res->boolean_field, false);

    set_number(val, 2.5);
    res = convert_to_boolean(val);
    EXPECT_EQ(res->flag, Type::Boolean);
    EXPECT_EQ(res->boolean_field, true);

    set_number(val, 0);
    res = convert_to_boolean(val);
    EXPECT_EQ(res->flag, Type::Boolean);
    EXPECT_EQ(res->boolean_field, false);

    set_string(val, "Hello world");
    res = convert_to_boolean(val);
    EXPECT_EQ(res->flag, Type::Boolean);
    EXPECT_EQ(res->boolean_field, true);

    set_string(val, "");
    res = convert_to_boolean(val);
    EXPECT_EQ(res->flag, Type::Boolean);
    EXPECT_EQ(res->boolean_field, false);

    Object object;
    set_object(val, object);
    res = convert_to_boolean(val);
    EXPECT_EQ(res->flag, Type::Boolean);
    EXPECT_EQ(res->boolean_field, true);
}

TEST(VariableTest, convert_to_number_test)
{
    Variable *res;
    Variable *val = variable_allocate();

    set_undefined(val);
    res = convert_to_number(val);
    EXPECT_EQ(res->flag, Type::NaN);

    set_null(val);
    res = convert_to_number(val);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, 0);

    set_nan(val);
    res = convert_to_number(val);
    EXPECT_EQ(res->flag, Type::NaN);

    set_infinity(val);
    res = convert_to_number(val);
    EXPECT_EQ(res->flag, Type::Infinity);

    set_neginfinity(val);
    res = convert_to_number(val);
    EXPECT_EQ(res->flag, Type::NegInfinity);

    set_boolean(val, true);
    res = convert_to_number(val);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, 1);

    set_boolean(val, false);
    res = convert_to_number(val);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, 0);

    set_number(val, 2.5);
    res = convert_to_number(val);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, 2.5);

    set_number(val, 0);
    res = convert_to_number(val);
    EXPECT_EQ(res->flag, Type::Number);
    EXPECT_EQ(res->number_field, 0);

    set_string(val, "Hello world");
    res = convert_to_number(val);
    EXPECT_EQ(res->flag, Type::NaN);

    set_string(val, "");
    res = convert_to_number(val);
    EXPECT_EQ(res->flag, Type::NaN);

    Object object;
    set_object(val, object);
    res = convert_to_number(val);
    EXPECT_EQ(res->flag, Type::NaN);
}

TEST(VariableTest, convert_to_string_test)
{
    Variable *res;
    Variable *val = variable_allocate();

    set_undefined(val);
    res = convert_to_string(val);
    EXPECT_EQ(res->flag, Type::String);
    EXPECT_EQ(res->string_field, "undefined");

    set_null(val);
    res = convert_to_string(val);
    EXPECT_EQ(res->flag, Type::String);
    EXPECT_EQ(res->string_field, "null");

    set_nan(val);
    res = convert_to_string(val);
    EXPECT_EQ(res->flag, Type::String);
    EXPECT_EQ(res->string_field, "NaN");

    set_infinity(val);
    res = convert_to_string(val);
    EXPECT_EQ(res->flag, Type::String);
    EXPECT_EQ(res->string_field, "Infinity");

    set_neginfinity(val);
    res = convert_to_string(val);
    EXPECT_EQ(res->flag, Type::String);
    EXPECT_EQ(res->string_field, "-Infinity");

    set_boolean(val, true);
    res = convert_to_string(val);
    EXPECT_EQ(res->flag, Type::String);
    EXPECT_EQ(res->string_field, "true");

    set_boolean(val, false);
    res = convert_to_string(val);
    EXPECT_EQ(res->flag, Type::String);
    EXPECT_EQ(res->string_field, "false");

    set_number(val, 2.5);
    res = convert_to_string(val);
    EXPECT_EQ(res->flag, Type::String);
    EXPECT_EQ(res->string_field, "2.500000");

    set_number(val, 0);
    res = convert_to_string(val);
    EXPECT_EQ(res->flag, Type::String);
    EXPECT_EQ(res->string_field, "0.000000");

    set_string(val, "Hello world");
    res = convert_to_string(val);
    EXPECT_EQ(res->flag, Type::String);
    EXPECT_EQ(res->string_field, "Hello world");

    set_string(val, "");
    res = convert_to_string(val);
    EXPECT_EQ(res->flag, Type::String);
    EXPECT_EQ(res->string_field, "");

    Object object;
    set_object(val, object);
    res = convert_to_string(val);
    EXPECT_EQ(res->flag, Type::String);
    EXPECT_EQ(res->string_field, "{}");
}