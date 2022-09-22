#ifndef C_VARIABLE_H
#define C_VARIABLE_H

#include <stdint.h>

typedef struct Variable Variable;

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

Variable *convert_to_boolean(Variable *val);
Variable *convert_to_number(Variable *val);

void print(Variable *self);

#endif