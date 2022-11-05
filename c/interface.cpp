
#include "variable/variable.hpp"
#include <assert.h>

extern "C"
{
#include "interface.h"
}

Variable *allocate()
{
    Variable *res = new Variable();
    return res;
}

void deallocate(Variable *)
{
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

    self->set_number(NumberType::NaN);
}

void set_object(Variable *self)
{
    assert(self != nullptr);

    self->set_object(Object());
}

void set_array(Variable *self)
{
    assert(self != nullptr);

    self->set_array(Array(std::vector<Variable *>{}));
}

void set_infinity(Variable *self)
{
    assert(self != nullptr);

    self->set_number(NumberType::Infinity);
}

void set_neginfinity(Variable *self)
{
    assert(self != nullptr);

    self->set_number(NumberType::NegInfinity);
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

void add_property_by_str(Variable *self, const char *key, Variable *val)
{
    assert(self != nullptr);
    assert(key != nullptr);

    self->add_property(std::string(key), val);
}

void add_property_by_var(Variable *self, Variable *key, Variable *val)
{
    assert(self != nullptr);
    assert(key != nullptr);

    self->add_property(*key, val);
}

Variable *get_property_by_str(Variable *self, const char *key, uint8_t allocate)
{
    assert(self != nullptr);
    assert(key != nullptr);

    return self->get_property(std::string(key), allocate);
}

Variable *get_property_by_var(Variable *self, Variable *key, uint8_t allocate)
{
    assert(self != nullptr);
    assert(key != nullptr);

    return self->get_property(*key, allocate);
}

void remove_property(Variable *self, const char *key)
{
    assert(self != nullptr);
    assert(key != nullptr);

    self->remove_property(key);
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

Variable *arithmetic_addition(Variable *val1, Variable *val2)
{
    assert(val1 != nullptr);
    assert(val2 != nullptr);

    Variable *ret = new Variable();
    *ret = *val1 + *val2;
    return ret;
}

Variable *arithmetic_substraction(Variable *val1, Variable *val2)
{
    assert(val1 != nullptr);
    assert(val2 != nullptr);

    Variable *ret = new Variable();
    *ret = *val1 - *val2;
    return ret;
}

Variable *arithmetic_multiplication(Variable *val1, Variable *val2)
{
    assert(val1 != nullptr);
    assert(val2 != nullptr);

    Variable *ret = new Variable();
    *ret = *val1 * *val2;
    return ret;
}

Variable *arithmetic_division(Variable *val1, Variable *val2)
{
    assert(val1 != nullptr);
    assert(val2 != nullptr);

    Variable *ret = new Variable();
    *ret = *val1 / *val2;
    return ret;
}

Variable *logical_not(Variable *val)
{
    assert(val != nullptr);

    Variable *ret = new Variable();
    ret->set_boolean(!*val);
    return ret;
}

Variable *logical_and(Variable *val1, Variable *val2)
{
    assert(val1 != nullptr);
    assert(val2 != nullptr);

    Variable *ret = new Variable();
    *ret = *val1 && *val2;
    return ret;
}

Variable *logical_or(Variable *val1, Variable *val2)
{
    assert(val1 != nullptr);
    assert(val2 != nullptr);

    Variable *ret = new Variable();
    *ret = *val1 || *val2;
    return ret;
}

Variable *logical_eq(Variable *val1, Variable *val2)
{
    // TODO implement
    assert(val1 != nullptr);
    assert(val2 != nullptr);

    Variable *ret = new Variable();
    ret->set_boolean(*val1 == *val2);
    return ret;
}

Variable *logical_ne(Variable *val1, Variable *val2)
{
    // TODO implement
    assert(val1 != nullptr);
    assert(val2 != nullptr);

    Variable *ret = new Variable();
    ret->set_boolean(*val1 != *val2);
    return ret;
}

Variable *logical_seq(Variable *val1, Variable *val2)
{
    assert(val1 != nullptr);
    assert(val2 != nullptr);

    Variable *ret = new Variable();
    ret->set_boolean(*val1 == *val2);
    return ret;
}

Variable *logical_sne(Variable *val1, Variable *val2)
{
    assert(val1 != nullptr);
    assert(val2 != nullptr);

    Variable *ret = new Variable();
    ret->set_boolean(*val1 != *val2);
    return ret;
}

Variable *logical_gt(Variable *val1, Variable *val2)
{
    assert(val1 != nullptr);
    assert(val2 != nullptr);

    Variable *ret = new Variable();
    ret->set_boolean(*val1 > *val2);
    return ret;
}

Variable *logical_ge(Variable *val1, Variable *val2)
{
    assert(val1 != nullptr);
    assert(val2 != nullptr);

    Variable *ret = new Variable();
    ret->set_boolean(*val1 >= *val2);
    return ret;
}

Variable *logical_lt(Variable *val1, Variable *val2)
{
    assert(val1 != nullptr);
    assert(val2 != nullptr);

    Variable *ret = new Variable();
    ret->set_boolean(*val1 < *val2);
    return ret;
}

Variable *logical_le(Variable *val1, Variable *val2)
{
    assert(val1 != nullptr);
    assert(val2 != nullptr);

    Variable *ret = new Variable();
    ret->set_boolean(*val1 <= *val2);
    return ret;
}

void variable_assert(Variable *val)
{
    assert(val != nullptr);

    if (!val->to_boolean())
    {
        abort();
    }
}

void variable_assert_eq(Variable *val1, Variable *val2)
{
    assert(val1 != nullptr);
    assert(val2 != nullptr);

    if (*val1 != *val2)
    {
        abort();
    }
}

void print(Variable *self)
{
    assert(self != nullptr);

    printf("%s\n", self->to_string().c_str());
}