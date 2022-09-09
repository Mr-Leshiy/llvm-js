#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>

#include "variable.h"

VariableType *allocate()
{
        VariableType *res = (VariableType *)malloc(sizeof(VariableType));
        // setup default value
        res->flag = 0;
        return res;
}
void set_number(VariableType *self, double val)
{
        assert(self != NULL);

        self->flag = Number;
        self->number_field = val;
}
void set_boolean(VariableType *self, uint8_t val)
{
        assert(self != NULL);

        self->flag = Boolean;
        self->boolean_field = val;
}
void set_string(VariableType *self, const char *val)
{
        assert(self != NULL);

        self->flag = String;
        self->string_field = val;
}
void set_variable(VariableType *self, VariableType *val)
{
        assert(val != NULL);

        switch (val->flag)
        {
        case Number:
                set_number(self, val->number_field);
                break;
        case Boolean:
                set_boolean(self, val->boolean_field);
                break;
        case String:
                set_string(self, strdup(val->string_field));
                break;
        default:
                assert(0);
                break;
        }
}