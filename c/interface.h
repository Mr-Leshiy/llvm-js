#ifndef C_INTERFACE_H
#define C_INTERFACE_H

#include <stdint.h>

typedef struct Variable Variable;

Variable *allocate();
void deallocate(Variable *);

void set_undefined(Variable *self);
void set_null(Variable *self);
void set_nan(Variable *self);
void set_object(Variable *self);
void set_array(Variable *self);
void set_infinity(Variable *self);
void set_neginfinity(Variable *self);
void set_number(Variable *self, double val);
void set_boolean(Variable *self, uint8_t val);
void set_string(Variable *self, const char *val);
void set_variable(Variable *self, Variable *val);

void add_property_by_str(Variable *self, const char *key, Variable *val);
void add_property_by_var(Variable *self, Variable *key, Variable *val);
Variable *get_property_by_str(Variable *self, const char *key, uint8_t allocate);
Variable *get_property_by_var(Variable *self, Variable *key, uint8_t allocate);
void remove_property(Variable *self, const char *key);

uint8_t get_boolean(Variable *self);

Variable *convert_to_boolean(Variable *val);
Variable *convert_to_number(Variable *val);
Variable *convert_to_string(Variable *val);

Variable *arithmetic_addition(Variable *val1, Variable *val2);
Variable *arithmetic_substraction(Variable *val1, Variable *val2);
Variable *arithmetic_multiplication(Variable *val1, Variable *val2);
Variable *arithmetic_division(Variable *val1, Variable *val2);

Variable *logical_not(Variable *val);
Variable *logical_and(Variable *val1, Variable *val2);
Variable *logical_or(Variable *val1, Variable *val2);
Variable *logical_eq(Variable *val1, Variable *val2);
Variable *logical_ne(Variable *val1, Variable *val2);
Variable *logical_seq(Variable *val1, Variable *val2);
Variable *logical_sne(Variable *val1, Variable *val2);
Variable *logical_gt(Variable *val1, Variable *val2);
Variable *logical_ge(Variable *val1, Variable *val2);
Variable *logical_lt(Variable *val1, Variable *val2);
Variable *logical_le(Variable *val1, Variable *val2);

void variable_assert(Variable *val);
void variable_assert_eq(Variable *val1, Variable *val2);

void print(Variable *self);

#endif