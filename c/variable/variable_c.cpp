#include "variable.hpp"

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

    self->number_field = Number(NumberType::NaN);
}

void set_object(Variable *self)
{
    assert(self != nullptr);

    self->set_object(Object());
}

void set_infinity(Variable *self)
{
    assert(self != nullptr);

    self->number_field = Number(NumberType::Infinity);
}

void set_neginfinity(Variable *self)
{
    assert(self != nullptr);

    self->number_field = Number(NumberType::NegInfinity);
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

    return self->to_boolean();
}

Variable *convert_to_boolean(Variable *val)
{
    assert(val != nullptr);

    Variable *ret = new Variable();
    ret->set_boolean(val->to_boolean());
    return ret;
}

Variable *convert_to_number(Variable *val)
{
    assert(val != nullptr);

    Variable *ret = new Variable();
    ret->set_number(val->to_number());
    return ret;
}

Variable *convert_to_string(Variable *val)
{
    assert(val != nullptr);

    Variable *ret = new Variable();
    ret->set_string(val->to_string());
    return ret;
}

void print(Variable *self)
{
    assert(self != nullptr);

    printf("%s\n", self->to_string().c_str());
}