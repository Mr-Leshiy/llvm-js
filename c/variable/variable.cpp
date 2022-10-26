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

bool Variable::to_boolean() const
{
    switch (this->flag)
    {
    case Type::Undefined:
        return false;
        break;
    case Type::Null:
        return false;
        break;
    case Type::NaN:
        return false;
        break;
    case Type::Infinity:
        return true;
        break;
    case Type::NegInfinity:
        return true;
        break;
    case Type::Number:
        return this->number_field != 0;
        break;
    case Type::Boolean:
        return this->boolean_field;
        break;
    case Type::String:
        return !this->string_field.empty();
        break;
    case Type::Object:
        return true;
        break;
    case Type::Array:
        return false;
        break;
    default:
        assert(0);
        break;
    }
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
