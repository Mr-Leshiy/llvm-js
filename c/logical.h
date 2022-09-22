#ifndef C_LOGICAL_H
#define C_LOGICAL_H

#include "variable.h"

Variable *logical_not(Variable *val);
Variable *logical_and(Variable *val1, Variable *val2);
Variable *logical_or(Variable *val1, Variable *val2);
Variable *logical_eq(Variable *val1, Variable *val2);
Variable *logical_ne(Variable *val1, Variable *val2);
Variable *logical_seq(Variable *val1, Variable *val2);
Variable *logical_sne(Variable *val1, Variable *val2);

#endif