#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "variable.hpp"
#include "assert/assert.hpp"

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
    this->object_field.clear();
    this->object_field = val.object_field;
    this->array_field.clear();
    this->array_field = val.array_field;
    this->function_field = val.function_field;
    return *this;
}

Variable &Variable::set_undefined()
{
    this->flag = Type::Undefined;
    return *this;
}

Variable &Variable::set_null()
{
    this->flag = Type::Null;
    return *this;
}

Variable &Variable::set_number(const Number &val)
{
    this->flag = Type::Number;
    this->number_field = val;
    return *this;
}

Variable &Variable::set_boolean(bool val)
{
    this->flag = Type::Boolean;
    this->boolean_field = val;
    return *this;
}

Variable &Variable::set_string(const std::string &val)
{
    this->flag = Type::String;
    this->string_field = val;
    return *this;
}

Variable &Variable::set_object(const Object &val)
{
    this->flag = Type::Object;
    this->object_field = val;
    return *this;
}

Variable &Variable::set_array(const Array &val)
{
    this->flag = Type::Array;
    this->array_field = val;
    return *this;
}

Variable &Variable::set_function(const Function &val)
{
    this->flag = Type::Function;
    this->function_field = val;
    return *this;
}

Type Variable::get_flag() const
{
    return this->flag;
}

bool Variable::get_boolean() const
{
    return this->boolean_field;
}

const std::string &Variable::get_string() const
{
    return this->string_field;
}

const Number &Variable::get_number() const
{
    return this->number_field;
}

const Object &Variable::get_object() const
{
    return this->object_field;
}

const Array &Variable::get_array() const
{
    return this->array_field;
}

const Function &Variable::get_function() const
{
    return this->function_field;
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
    case Type::Number:
        return this->number_field.to_boolean();
        break;
    case Type::Boolean:
        return this->boolean_field;
        break;
    case Type::String:
        return !this->string_field.empty();
        break;
    case Type::Object:
        return !this->object_field.empty();
        break;
    case Type::Array:
        return !this->array_field.empty();
        break;
    case Type::Function:
        return true;
        break;
    default:
        ASSERT(false);
        break;
    }
}

Number Variable::to_number() const
{
    switch (this->flag)
    {
    case Type::Undefined:
        return Number(NumberType::NaN);
        break;
    case Type::Null:
        return Number(0);
        break;
    case Type::Number:
        return this->number_field;
        break;
    case Type::Boolean:
        return Number(this->boolean_field ? 1 : 0);
        break;
    case Type::String:
        return Number(NumberType::NaN);
        break;
    case Type::Object:
        return Number(NumberType::NaN);
        break;
    case Type::Array:
        return Number(NumberType::NaN);
        break;
    case Type::Function:
        return Number(NumberType::NaN);
        break;
    default:
        ASSERT(false);
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
    case Type::Number:
        return this->number_field.to_string();
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
        break;
    case Type::Function:
        return this->function_field.to_string();
        break;
    default:
        ASSERT(false);
        break;
    }
}

void Variable::add_property(const std::string &key, Variable *val)
{
    // TODO print runtime error message
    if (this->flag == Type::Object)
    {
        this->object_field.add_property(key, val);
    }
    if (this->flag == Type::Array)
    {
        this->array_field.put(*val, Number(std::stod(key)));
    }
}

void Variable::add_property(const Variable &key, Variable *val)
{
    // TODO print runtime error message
    if (this->flag == Type::Object)
    {
        this->object_field.add_property(key.to_string(), val);
    }
    if (this->flag == Type::Array)
    {
        this->array_field.put(*val, key.to_number());
    }
}

Variable *Variable::get_property(const std::string &key, bool allocate)
{
    if (this->flag == Type::Object)
    {
        return this->object_field.get_property(key, allocate);
    }
    if (this->flag == Type::Array)
    {
        return this->array_field.get(Number(std::stod(key)), allocate);
    }
    return GarbageCollector<Variable>::get_instance().allocate();
}

Variable *Variable::get_property(const Variable &key, bool allocate)
{
    if (this->flag == Type::Object)
    {
        return this->object_field.get_property(key.to_string(), allocate);
    }
    if (this->flag == Type::Array)
    {
        return this->array_field.get(key.to_number(), allocate);
    }
    return GarbageCollector<Variable>::get_instance().allocate();
}

void Variable::remove_property(const std::string &key)
{
    // TODO print runtime error message
    if (this->flag == Type::Object)
    {
        this->object_field.remove_property(key);
    }
}

Variable operator+(const Variable &a, const Variable &b)
{
    Variable ret;
    if (a.flag == Type::String || b.flag == Type::String)
    {
        ret.set_string(a.to_string() + b.to_string());
    }
    else
    {
        ret.set_number(a.to_number() + b.to_number());
    }
    return ret;
}

Variable operator-(const Variable &a, const Variable &b)
{
    Variable ret;
    ret.set_number(a.to_number() - b.to_number());
    return ret;
}

Variable operator*(const Variable &a, const Variable &b)
{
    Variable ret;
    ret.set_number(a.to_number() * b.to_number());
    return ret;
}

Variable operator/(const Variable &a, const Variable &b)
{
    Variable ret;
    ret.set_number(a.to_number() / b.to_number());
    return ret;
}

bool operator==(const Variable &a, const Variable &b)
{
    switch (a.flag)
    {
    case Type::Undefined:
        return b.flag == Type::Undefined;
        break;
    case Type::Null:
        return b.flag == Type::Null;
        break;
    case Type::Number:
        if (b.flag == Type::Number)
        {
            return a.number_field == b.number_field;
        }
        else
        {
            return false;
        }
        break;
    case Type::Boolean:
        if (b.flag == Type::Boolean)
        {
            return a.boolean_field == b.boolean_field;
        }
        else
        {
            return false;
        }
        break;
    case Type::String:
        if (b.flag == Type::String)
        {
            return a.string_field == b.string_field;
        }
        else
        {
            return false;
        }
        break;
    case Type::Object:
        return false;
        break;
    case Type::Array:
        return false;
        break;
    case Type::Function:
        return false;
        break;
    default:
        ASSERT(false);
        break;
    }
}

bool operator!=(const Variable &a, const Variable &b)
{
    return !(a == b);
}

bool operator>(const Variable &a, const Variable &b)
{
    switch (a.flag)
    {
    case Type::Number:
        switch (b.flag)
        {
        case Type::Number:
            return a.number_field > b.number_field;
            break;
        case Type::Boolean:
            return a.number_field > Number(b.boolean_field);
            break;
        case Type::Null:
            return a.number_field > Number(0);
            break;
        case Type::String:
            break;
        case Type::Undefined:
            break;
        case Type::Object:
            break;
        case Type::Array:
            break;
        case Type::Function:
            break;
        default:
            ASSERT(false);
            break;
        }
        break;
    case Type::Boolean:
        switch (b.flag)
        {
        case Type::Number:
            return Number(a.boolean_field) > b.number_field;
            break;
        case Type::Boolean:
            return a.boolean_field > b.boolean_field;
            break;
        case Type::Null:
            return a.boolean_field > 0;
            break;
        case Type::String:
            break;
        case Type::Undefined:
            break;
        case Type::Object:
            break;
        case Type::Array:
            break;
        case Type::Function:
            break;
        default:
            ASSERT(false);
            break;
        }
        break;
    case Type::Null:
        switch (b.flag)
        {
        case Type::Number:
            return Number(0) > b.number_field;
            break;
        case Type::Boolean:
            return false;
            break;
        case Type::Null:
            return false;
            break;
        case Type::String:
            break;
        case Type::Undefined:
            break;
        case Type::Object:
            break;
        case Type::Array:
            break;
        case Type::Function:
            break;
        default:
            ASSERT(false);
            break;
        }
        break;
    case Type::String:
        switch (b.flag)
        {
        case Type::String:
            return a.string_field > b.string_field;
            break;
        case Type::Number:
            break;
        case Type::Boolean:
            break;
        case Type::Null:
            break;
        case Type::Undefined:
            break;
        case Type::Object:
            break;
        case Type::Array:
            break;
        case Type::Function:
            break;
        default:
            ASSERT(false);
            break;
        }
        break;
    case Type::Undefined:
        break;
    case Type::Object:
        break;
    case Type::Array:
        break;
    case Type::Function:
        break;
    default:
        ASSERT(false);
        break;
    }
    return false;
}

bool operator>=(const Variable &a, const Variable &b)
{
    switch (a.flag)
    {
    case Type::Number:
        switch (b.flag)
        {
        case Type::Number:
            return a.number_field >= b.number_field;
            break;
        case Type::Boolean:
            return a.number_field >= Number(b.boolean_field);
            break;
        case Type::Null:
            return a.number_field >= Number(0);
            break;
        case Type::String:
            break;
        case Type::Undefined:
            break;
        case Type::Object:
            break;
        case Type::Array:
            break;
        case Type::Function:
            break;
        default:
            ASSERT(false);
            break;
        }
        break;
    case Type::Boolean:
        switch (b.flag)
        {
        case Type::Number:
            return Number(a.boolean_field) >= b.number_field;
            break;
        case Type::Boolean:
            return a.boolean_field >= b.boolean_field;
            break;
        case Type::Null:
            return true;
            break;
        case Type::String:
            break;
        case Type::Undefined:
            break;
        case Type::Object:
            break;
        case Type::Array:
            break;
        case Type::Function:
            break;
        default:
            ASSERT(false);
            break;
        }
        break;
    case Type::Null:
        switch (b.flag)
        {
        case Type::Number:
            return Number(0) >= b.number_field;
            break;
        case Type::Boolean:
            return 0 >= b.boolean_field;
            break;
        case Type::Null:
            return true;
            break;
        case Type::String:
            break;
        case Type::Undefined:
            break;
        case Type::Object:
            break;
        case Type::Array:
            break;
        case Type::Function:
            break;
        default:
            ASSERT(false);
            break;
        }
        break;
    case Type::String:
        switch (b.flag)
        {
        case Type::String:
            return a.string_field >= b.string_field;
            break;
        case Type::Number:
            break;
        case Type::Boolean:
            break;
        case Type::Null:
            break;
        case Type::Undefined:
            break;
        case Type::Object:
            break;
        case Type::Array:
            break;
        case Type::Function:
            break;
        default:
            ASSERT(false);
            break;
        }
        break;
    case Type::Undefined:
        break;
    case Type::Object:
        break;
    case Type::Array:
        break;
    case Type::Function:
        break;
    default:
        ASSERT(false);
        break;
    }
    return false;
}

bool operator<(const Variable &a, const Variable &b)
{
    switch (a.flag)
    {
    case Type::Number:
        switch (b.flag)
        {
        case Type::Number:
            return a.number_field < b.number_field;
            break;
        case Type::Boolean:
            return a.number_field < Number(b.boolean_field);
            break;
        case Type::Null:
            return a.number_field < Number(0);
            break;
        case Type::String:
            break;
        case Type::Undefined:
            break;
        case Type::Object:
            break;
        case Type::Array:
            break;
        case Type::Function:
            break;
        default:
            ASSERT(false);
            break;
        }
        break;
    case Type::Boolean:
        switch (b.flag)
        {
        case Type::Number:
            return Number(a.boolean_field) < b.number_field;
            break;
        case Type::Boolean:
            return a.boolean_field < b.boolean_field;
            break;
        case Type::Null:
            return false;
            break;
        case Type::String:
            break;
        case Type::Undefined:
            break;
        case Type::Object:
            break;
        case Type::Array:
            break;
        case Type::Function:
            break;
        default:
            ASSERT(false);
            break;
        }
        break;
    case Type::Null:
        switch (b.flag)
        {
        case Type::Number:
            return Number(0) < b.number_field;
            break;
        case Type::Boolean:
            return 0 < b.boolean_field;
            break;
        case Type::Null:
            return false;
            break;
        case Type::String:
            break;
        case Type::Undefined:
            break;
        case Type::Object:
            break;
        case Type::Array:
            break;
        case Type::Function:
            break;
        default:
            ASSERT(false);
            break;
        }
        break;
    case Type::String:
        switch (b.flag)
        {
        case Type::String:
            return a.string_field < b.string_field;
            break;
        case Type::Number:
            break;
        case Type::Boolean:
            break;
        case Type::Null:
            break;
        case Type::Undefined:
            break;
        case Type::Object:
            break;
        case Type::Array:
            break;
        case Type::Function:
            break;
        default:
            ASSERT(false);
            break;
        }
        break;
    case Type::Undefined:
        break;
    case Type::Object:
        break;
    case Type::Array:
        break;
    case Type::Function:
        break;
    default:
        ASSERT(false);
        break;
    }
    return false;
}

bool operator<=(const Variable &a, const Variable &b)
{
    switch (a.flag)
    {
    case Type::Number:
        switch (b.flag)
        {
        case Type::Number:
            return a.number_field <= b.number_field;
            break;
        case Type::Boolean:
            return a.number_field <= Number(b.boolean_field);
            break;
        case Type::Null:
            return a.number_field <= Number(0);
            break;
        case Type::String:
            break;
        case Type::Undefined:
            break;
        case Type::Object:
            break;
        case Type::Array:
            break;
        case Type::Function:
            break;
        default:
            ASSERT(false);
            break;
        }
        break;
    case Type::Boolean:
        switch (b.flag)
        {
        case Type::Number:
            return Number(a.boolean_field) <= b.number_field;
            break;
        case Type::Boolean:
            return a.boolean_field <= b.boolean_field;
            break;
        case Type::Null:
            return a.boolean_field <= 0;
            break;
        case Type::String:
            break;
        case Type::Undefined:
            break;
        case Type::Object:
            break;
        case Type::Array:
            break;
        case Type::Function:
            break;
        default:
            ASSERT(false);
            break;
        }
        break;
    case Type::Null:
        switch (b.flag)
        {
        case Type::Number:
            return Number(0) <= b.number_field;
            break;
        case Type::Boolean:
            return true;
            break;
        case Type::Null:
            return true;
            break;
        case Type::String:
            break;
        case Type::Undefined:
            break;
        case Type::Object:
            break;
        case Type::Array:
            break;
        case Type::Function:
            break;
        default:
            ASSERT(false);
            break;
        }
        break;
    case Type::String:
        switch (b.flag)
        {
        case Type::String:
            return a.string_field <= b.string_field;
            break;
        case Type::Number:
            break;
        case Type::Boolean:
            break;
        case Type::Null:
            break;
        case Type::Undefined:
            break;
        case Type::Object:
            break;
        case Type::Array:
            break;
        case Type::Function:
            break;
        default:
            ASSERT(false);
            break;
        }
        break;
    case Type::Undefined:
        break;
    case Type::Object:
        break;
    case Type::Array:
        break;
    case Type::Function:
        break;
    default:
        ASSERT(false);
        break;
    }
    return false;
}

bool operator!(const Variable &a)
{
    return !a.to_boolean();
}

Variable operator&&(const Variable &a, const Variable &b)
{
    Variable ret;
    if (a.to_boolean())
    {
        if (b.to_boolean())
        {
            ret = b;
        }
        else
        {
            ret = b;
        }
    }
    else
    {
        ret = a;
    }
    return ret;
}

Variable operator||(const Variable &a, const Variable &b)
{
    Variable ret;
    if (a.to_boolean())
    {
        ret = a;
    }
    else
    {
        if (b.to_boolean())
        {
            ret = b;
        }
        else
        {
            ret = b;
        }
    }
    return ret;
}
