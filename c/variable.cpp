#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>

#include "variable.hpp"

Variable *allocate()
{
    Variable *res = new Variable;
    res->flag = Type::Undefined;
    return res;
}

void set_undefined(Variable *self)
{
    assert(self != nullptr);

    self->flag = Type::Undefined;
}

void set_null(Variable *self)
{
    assert(self != nullptr);

    self->flag = Type::Null;
}

void set_nan(Variable *self)
{
    assert(self != nullptr);

    self->flag = Type::NaN;
}

void set_infinity(Variable *self)
{
    assert(self != nullptr);

    self->flag = Type::Infinity;
}

void set_neginfinity(Variable *self)
{
    assert(self != nullptr);

    self->flag = Type::NegInfinity;
}

void set_number(Variable *self, double val)
{
    assert(self != nullptr);

    self->flag = Type::Number;
    self->number_field = val;
}

void set_boolean(Variable *self, uint8_t val)
{
    assert(self != nullptr);

    self->flag = Type::Boolean;
    self->boolean_field = val;
}

void set_string(Variable *self, const char *val)
{
    assert(self != nullptr);

    self->flag = Type::String;
    self->string_field = val;
}

void set_variable(Variable *self, Variable *val)
{
    assert(self != nullptr);
    assert(val != nullptr);

    switch (val->flag)
    {
    case Type::Undefined:
        set_undefined(self);
        break;
    case Type::Null:
        set_null(self);
        break;
    case Type::NaN:
        set_nan(self);
        break;
    case Type::Infinity:
        set_infinity(self);
        break;
    case Type::NegInfinity:
        set_neginfinity(self);
        break;
    case Type::Number:
        set_number(self, val->number_field);
        break;
    case Type::Boolean:
        set_boolean(self, val->boolean_field);
        break;
    case Type::String:
        set_string(self, val->string_field.c_str());
        break;
    default:
        assert(0);
        break;
    }
}

uint8_t get_boolean(Variable *self)
{
    assert(self != nullptr);
    assert(self->flag == Type::Boolean);

    return self->boolean_field;
}

Variable *convert_to_boolean(Variable *val)
{
    assert(val != nullptr);

    Variable *ret = allocate();
    switch (val->flag)
    {
    case Type::Undefined:
        set_boolean(ret, false);
        break;
    case Type::Null:
        set_boolean(ret, false);
        break;
    case Type::NaN:
        set_boolean(ret, false);
        break;
    case Type::Infinity:
        set_boolean(ret, true);
        break;
    case Type::NegInfinity:
        set_boolean(ret, true);
        break;
    case Type::Number:
        set_boolean(ret, val->number_field != 0);
        break;
    case Type::Boolean:
        set_boolean(ret, val->boolean_field);
        break;
    case Type::String:
        set_boolean(ret, !val->string_field.empty());
        break;
    default:
        assert(0);
        break;
    }
    return ret;
}

Variable *convert_to_number(Variable *val)
{
    assert(val != nullptr);

    Variable *ret = allocate();
    switch (val->flag)
    {
    case Type::Undefined:
        set_nan(ret);
        break;
    case Type::Null:
        set_number(ret, 0);
        break;
    case Type::NaN:
        set_nan(ret);
        break;
    case Type::Infinity:
        set_infinity(ret);
        break;
    case Type::NegInfinity:
        set_neginfinity(ret);
        break;
    case Type::Number:
        set_number(ret, val->number_field);
        break;
    case Type::Boolean:
        set_number(ret, val->boolean_field ? 1 : 0);
        break;
    case Type::String:
        set_nan(ret);
        break;
    default:
        assert(0);
        break;
    }
    return ret;
}

Variable *convert_to_string(Variable *val)
{
    assert(val != nullptr);

    Variable *ret = allocate();
    switch (val->flag)
    {
    case Type::Undefined:
        set_string(ret, "undefined");
        break;
    case Type::Null:
        set_string(ret, "null");
        break;
    case Type::NaN:
        set_string(ret, "NaN");
        break;
    case Type::Infinity:
        set_string(ret, "Infinity");
        break;
    case Type::NegInfinity:
        set_string(ret, "-Infinity");
        break;
    case Type::Number:
        set_string(ret, std::to_string(val->number_field).c_str());
        break;
    case Type::Boolean:
        set_string(ret, val->boolean_field ? "true" : "false");
        break;
    case Type::String:
        set_string(ret, val->string_field.c_str());
        break;
    default:
        assert(0);
        break;
    }
    return ret;
}

void print(Variable *self)
{
    assert(self != nullptr);

    switch (self->flag)
    {
    case Type::Undefined:
        printf("undefined\n");
        break;
    case Type::Null:
        printf("null\n");
        break;
    case Type::NaN:
        printf("NaN\n");
        break;
    case Type::Infinity:
        printf("Infinity\n");
        break;
    case Type::NegInfinity:
        printf("-Infinity\n");
        break;
    case Type::Number:
        printf("%f\n", self->number_field);
        break;
    case Type::Boolean:
        printf("%s\n", self->boolean_field ? "true" : "false");
        break;
    case Type::String:
        printf("%s\n", self->string_field.c_str());
        break;
    default:
        assert(0);
        break;
    }
}