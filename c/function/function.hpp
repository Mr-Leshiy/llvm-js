#ifndef C_FUNCTION_HPP
#define C_FUNCTION_HPP

#include <stdint.h>
#include <string>

struct Variable;

typedef Variable *(*FUNC_TYPE)(Variable **);

struct Function
{
    Function() = default;
    Function(FUNC_TYPE func, uint32_t args_num) : func(func), args_num(args_num) {}

    Variable *call(Variable **args) const;

    std::string to_string() const;

    friend bool operator==(const Function &a, const Function &b);

private:
    FUNC_TYPE func{nullptr};
    uint32_t args_num{0};
};

#endif