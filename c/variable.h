#ifndef C_VARIABLE_H
#define C_VARIABLE_H

#include <stdint.h>

typedef struct VariableType
{
    uint8_t flag;
    double number_field;
    uint8_t boolean_field;
    const char* string_field;
} VariableType;

enum Type {
    Number = 1,
    Boolean = 2,
    String = 3,
};

VariableType *allocate();

void set_number(VariableType* self, double val);
void set_boolean(VariableType* self, uint8_t val);
void set_string(VariableType* self, const char* val);
void set_variable(VariableType* self, VariableType* val);

VariableType* convert_to_boolean(VariableType* val);

void print(VariableType* self);

#endif