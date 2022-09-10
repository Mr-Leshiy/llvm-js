#ifndef C_ASSERTIONS_H
#define C_ASSERTIONS_H

#include "variable.h"

void variable_assert(VariableType* var);
void variable_assert_eq(VariableType* var1, VariableType* var2);

#endif