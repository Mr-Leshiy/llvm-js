#ifndef C_VARIABLE_H
#define C_VARIABLE_H

#include <stdint.h>

typedef struct VariableType
{
    uint8_t flag;
    double number_field;
    uint8_t boolean_field;

} VariableType;

VariableType *allocate();

#endif