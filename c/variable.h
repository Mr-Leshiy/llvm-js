#ifndef C_VARIABLE_H
#define C_VARIABLE_H

#include <stdint.h>

typedef struct Variable Variable;
typedef struct Object Object;

Variable *allocate();

void set_undefined(Variable *self);
void set_null(Variable *self);
void set_nan(Variable *self);
void set_infinity(Variable *self);
void set_neginfinity(Variable *self);
void set_number(Variable *self, double val);
void set_boolean(Variable *self, uint8_t val);
void set_string(Variable *self, const char *val);
void set_variable(Variable *self, Variable *val);

// object
void init_object(Variable *self);
void add_property(Variable *self, const char *key, Variable *val);
Variable *get_property(Variable *self, const char *key);
void remove_property(Variable *self, const char *key);

uint8_t get_boolean(Variable *self);

Variable *convert_to_boolean(Variable *val);
Variable *convert_to_number(Variable *val);
Variable *convert_to_string(Variable *val);

void print(Variable *self);

#endif