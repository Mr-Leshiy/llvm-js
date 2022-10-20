#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>

#include "variable.hpp"
#include "object.hpp"

std::string Variable::to_string() const
{
    switch (this->flag)
    {
    case Type::Undefined:
        return "undefined";
        break;
    case Type::Null:
        return "null";
        break;
    case Type::NaN:
        return "NaN";
        break;
    case Type::Infinity:
        return "Infinity";
        break;
    case Type::NegInfinity:
        return "-Infinity";
        break;
    case Type::Number:
        return std::to_string(this->number_field);
        break;
    case Type::Boolean:
        return this->boolean_field ? "true" : "false";
        break;
    case Type::String:
        return this->string_field;
        break;
    case Type::Object:
        return this->object_field.to_string();
        break;
    default:
        assert(0);
        break;
    }
}

void Variable::set_object(Object &object)
{
    this->flag = Type::Object;
    this->object_field = object;
}

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

void set_object(Variable *self)
{
    assert(self != nullptr);

    self->flag = Type::Object;
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
    case Type::Object:
        self->set_object(val->object_field);
        break;
    default:
        assert(0);
        break;
    }
}

void add_property(Variable *self, const char *key, Variable *val)
{
    assert(self != nullptr);

    // TODO print runtime error message
    if (self->flag == Type::Object)
    {
        self->object_field.add_property(key, val);
    }
}

Variable *get_property_by_str(Variable *self, const char *key)
{
    assert(self != nullptr);

    return self->object_field.get_property(key);
}

Variable *get_property_by_var(Variable *self, Variable *key)
{
    assert(self != nullptr);
    assert(key != nullptr);

    return self->object_field.get_property(*key);
}

void remove_property(Variable *self, const char *key)
{
    assert(self != nullptr);

    self->object_field.remove_property(key);
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
    case Type::Object:
        set_boolean(ret, true);
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
    case Type::Object:
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
    set_string(ret, val->to_string().c_str());
    return ret;
}

void print(Variable *self)
{
    assert(self != nullptr);

    printf("%s\n", self->to_string().c_str());
}