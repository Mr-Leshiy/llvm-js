#ifndef C_ASSERTIONS_H
#define C_ASSERTIONS_H

#include "variable.h"

void variable_assert(VariableType *val);
void variable_assert_eq(VariableType *val1, VariableType *val2);

#endif