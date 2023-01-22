#ifndef C_FUNCTION_HPP
#define C_FUNCTION_HPP

#include <stdint.h>
#include <string.h>

struct Variable;

typedef Variable *(*FUNC_TYPE)(Variable **);

struct Function
{
    Function() = default;
    Function(FUNC_TYPE func, uint32_t args_num) : func(func), args_num(args_num) {}

    Variable *call(Variable **args)
    {
        return this->func(args);
    }

    std::string to_string() const
    {
        return "function, args num: " + std::to_string(this->args_num);
    }

    friend bool operator==(const Function &a, const Function &b)
    {
        return a.args_num == b.args_num && a.func == b.func;
    }

private:
    FUNC_TYPE func{nullptr};
    uint32_t args_num{0};
};

#endif