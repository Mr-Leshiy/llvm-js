#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>

#include "variable.h"

VariableType *allocate()
{
    VariableType *res = (VariableType *)malloc(sizeof(VariableType));
    res->flag = Undefined;
    return res;
}

void set_undefined(VariableType *self)
{
    assert(self != NULL);

    self->flag = Undefined;
}

void set_null(VariableType *self)
{
    assert(self != NULL);

    self->flag = Null;
}

void set_nan(VariableType *self)
{
    assert(self != NULL);

    self->flag = NaN;
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
    assert(self != NULL);
    assert(val != NULL);

    switch (val->flag)
    {
    case Undefined:
        set_undefined(self);
        break;
    case Null:
        set_null(self);
        break;
    case NaN:
        set_nan(self);
        break;
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

VariableType *convert_to_boolean(VariableType *val)
{
    assert(val != NULL);

    VariableType *ret = allocate();
    switch (val->flag)
    {
    case Undefined:
        set_boolean(ret, 0);
        break;
    case Null:
        set_boolean(ret, 0);
        break;
    case NaN:
        set_boolean(ret, 0);
        break;
    case Number:
        if (val->number_field == 0)
        {
            set_boolean(ret, 0);
        }
        else
        {
            set_boolean(ret, 1);
        }
        break;
    case Boolean:
        set_boolean(ret, val->boolean_field);
        break;
    case String:
        if (strlen(val->string_field) == 0)
        {
            set_boolean(ret, 0);
        }
        else
        {
            set_boolean(ret, 1);
        }
        break;
    default:
        assert(0);
        break;
    }
    return ret;
}

VariableType *convert_to_number(VariableType *val)
{
    assert(val != NULL);

    VariableType *ret = allocate();
    switch (val->flag)
    {
    case Undefined:
        set_nan(ret);
        break;
    case Null:
        set_number(ret, 0);
        break;
    case NaN:
        set_nan(ret);
        break;
    case Number:
        set_number(ret, val->number_field);
        break;
    case Boolean:
        set_number(ret, val->boolean_field ? 1 : 0 );
        break;
    case String:
        set_nan(ret);
        break;
    default:
        assert(0);
        break;
    }
    return ret;
}

void print(VariableType *self)
{
    assert(self != NULL);

    switch (self->flag)
    {
    case Undefined:
        printf("undefined\n");
        break;
    case Null:
        printf("null\n");
        break;
    case NaN:
        printf("NaN\n");
        break;
    case Number:
        printf("%f\n", self->number_field);
        break;
    case Boolean:
        if (self->boolean_field)
        {
            printf("true\n");
        }
        else
        {
            printf("false\n");
        }
        break;
    case String:
        printf("%s\n", self->string_field);
        break;
    default:
        assert(0);
        break;
    }
}