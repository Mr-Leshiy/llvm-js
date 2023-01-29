#include "function.hpp"
#include "variable/variable.hpp"
#include "garbage_collector/garbage_collector.hpp"

Variable *Function::call(std::vector<Variable *> args) const
{
    auto args_size = args.size();
    while (args.size() < this->args_num)
    {
        args.push_back(GarbageCollector<Variable>::get_instance().allocate());
    }

    auto ret = this->func(args.data());
    // for (auto i = args_size; i < args.size(); ++i)
    // {
    //     GarbageCollector<Variable>::get_instance().dec_counter(args[i]);
    // }
    return ret;
}

std::string Function::to_string() const
{
    return "Function, args num: " + std::to_string(this->args_num);
}

bool operator==(const Function &a, const Function &b)
{
    return a.args_num == b.args_num && a.func == b.func;
}