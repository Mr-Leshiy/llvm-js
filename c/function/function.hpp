#ifndef C_FUNCTION_HPP
#define C_FUNCTION_HPP

#include <stdint.h>

struct Variable;

typedef Variable *(*FUNC_TYPE)(Variable **);

struct Function
{
    Function(FUNC_TYPE func, uint32_t args_num) : func(func), args_num(args_num) {}

    Variable *call(Variable **args)
    {
        return this->func(args);
    }

private:
    FUNC_TYPE func;
    uint32_t args_num;
};

#endif