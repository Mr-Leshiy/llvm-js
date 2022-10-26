#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>

#include "variable.hpp"
#include "object.hpp"

Variable::Variable()
{
    this->flag = Type::Undefined;
}

Variable &Variable::operator=(const Variable &val)
{
    this->flag = val.flag;
    this->number_field = val.number_field;
    this->boolean_field = val.boolean_field;
    this->string_field = val.string_field;
    this->object_field = val.object_field;
    this->array_field = val.array_field;
    return *this;
}

void Variable::set_undefined()
{
    this->flag = Type::Undefined;
}

void Variable::set_null()
{
    this->flag = Type::Null;
}

void Variable::set_nan()
{
    this->flag = Type::NaN;
}

void Variable::set_infinity()
{
    this->flag = Type::Infinity;
}

void Variable::set_neginfinity()
{
    this->flag = Type::NegInfinity;
}

void Variable::set_number(double val)
{
    this->flag = Type::Number;
    this->number_field = val;
}

void Variable::set_boolean(bool val)
{
    this->flag = Type::Boolean;
    this->boolean_field = val;
}

void Variable::set_string(std::string val)
{
    this->flag = Type::String;
    this->string_field = val;
}

void Variable::set_object(const Object &val)
{
    this->flag = Type::Object;
    this->object_field = val;
}

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
    case Type::Array:
        return this->array_field.to_string();
    default:
        assert(0);
        break;
    }
}

// C wrappers

Variable *allocate()
{
    Variable *res = new Variable();
    return res;
}

void set_undefined(Variable *self)
{
    assert(self != nullptr);

    self->set_undefined();
}

void set_null(Variable *self)
{
    assert(self != nullptr);

    self->set_null();
}

void set_nan(Variable *self)
{
    assert(self != nullptr);

    self->set_nan();
}

void set_object(Variable *self)
{
    assert(self != nullptr);

    self->set_object(Object());
}

void set_infinity(Variable *self)
{
    assert(self != nullptr);

    self->set_infinity();
}

void set_neginfinity(Variable *self)
{
    assert(self != nullptr);

    self->set_neginfinity();
}

void set_number(Variable *self, double val)
{
    assert(self != nullptr);

    self->set_number(val);
}

void set_boolean(Variable *self, uint8_t val)
{
    assert(self != nullptr);

    self->set_boolean(val);
}

void set_string(Variable *self, const char *val)
{
    assert(self != nullptr);

    self->set_string(val);
}

void set_variable(Variable *self, Variable *val)
{
    assert(self != nullptr);
    assert(val != nullptr);

    *self = *val;
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
        ret->set_boolean(false);
        break;
    case Type::Null:
        ret->set_boolean(false);
        break;
    case Type::NaN:
        ret->set_boolean(false);
        break;
    case Type::Infinity:
        ret->set_boolean(true);
        break;
    case Type::NegInfinity:
        ret->set_boolean(true);
        break;
    case Type::Number:
        ret->set_boolean(val->number_field != 0);
        break;
    case Type::Boolean:
        ret->set_boolean(val->boolean_field);
        break;
    case Type::String:
        ret->set_boolean(!val->string_field.empty());
        break;
    case Type::Object:
        ret->set_boolean(true);
        break;
    case Type::Array:
        ret->set_boolean(false);
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
        ret->set_nan();
        break;
    case Type::Null:
        ret->set_number(0);
        break;
    case Type::NaN:
        ret->set_nan();
        break;
    case Type::Infinity:
        ret->set_infinity();
        break;
    case Type::NegInfinity:
        ret->set_neginfinity();
        break;
    case Type::Number:
        ret->set_number(val->number_field);
        break;
    case Type::Boolean:
        ret->set_number(val->boolean_field ? 1 : 0);
        break;
    case Type::String:
        ret->set_nan();
        break;
    case Type::Object:
        ret->set_nan();
        break;
    case Type::Array:
        ret->set_nan();
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
    ret->set_string(val->to_string());
    return ret;
}

void print(Variable *self)
{
    assert(self != nullptr);

    printf("%s\n", self->to_string().c_str());
}