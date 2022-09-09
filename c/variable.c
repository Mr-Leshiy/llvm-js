#include <stdio.h>
#include <stdlib.h>
#include "variable.h"

VariableType *allocate()
{
    printf("allocate VariableType \n");
    VariableType *res = (VariableType *)malloc(sizeof(VariableType));
    return res;
}
