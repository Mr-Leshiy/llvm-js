#ifndef C_ASSERTIONS_H
#define C_ASSERTIONS_H

#include "variable/variable.h"

void variable_assert(Variable *val);
void variable_assert_eq(Variable *val1, Variable *val2);

#endif