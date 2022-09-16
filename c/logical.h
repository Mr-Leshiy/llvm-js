#ifndef C_LOGICAL_H
#define C_LOGICAL_H

#include "variable.h"

VariableType *logical_not(VariableType *val);
VariableType *logical_and(VariableType *val1, VariableType *val2);
VariableType *logical_or(VariableType *val1, VariableType *val2);
VariableType *logical_eq(VariableType *val1, VariableType *val2);
VariableType *logical_ne(VariableType *val1, VariableType *val2);
VariableType *logical_seq(VariableType *val1, VariableType *val2);
VariableType *logical_sne(VariableType *val1, VariableType *val2);

#endif