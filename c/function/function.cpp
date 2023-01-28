#include "function.hpp"
#include "variable/variable.hpp"

Variable *Function::call(Variable **args) const
{
    return this->func(args);
}

std::string Function::to_string() const
{
    return "function, args num: " + std::to_string(this->args_num);
}

bool operator==(const Function &a, const Function &b)
{
    return a.args_num == b.args_num && a.func == b.func;
}