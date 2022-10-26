#ifndef C_ARITHMETIC_H
#define C_ARITHMETIC_H

#include "variable/variable.h"

Variable *arithmetic_addition(Variable *val1, Variable *val2);
Variable *arithmetic_substraction(Variable *val1, Variable *val2);
Variable *arithmetic_multiplication(Variable *val1, Variable *val2);
Variable *arithmetic_division(Variable *val1, Variable *val2);

#endif