#include "array.hpp"
#include "variable/variable.hpp"

void Array::push(Variable &value)
{
    this->values.push_back(&value);
}

Variable *Array::pop()
{
    if (this->values.empty())
    {
        return allocate();
    }

    auto *ret = this->values.back();
    this->values.pop_back();
    return ret;
}

Variable *Array::get(uint32_t index)
{
    if (index < this->len())
    {
        return this->values[index];
    }

    return new Variable();
}

Variable *Array::get(const Number &index)
{
    if (index.get_type() == NumberType::Number)
    {
        uint32_t i = index.get_value();
        if (i < this->len())
        {
            return this->values[i];
        }
    }

    return new Variable();
}

uint32_t Array::len() const
{
    return this->values.size();
}

std::string Array::to_string() const
{
    std::string res = "[";
    for (const auto &value : this->values)
    {
        res += value->to_string() + ",";
    }
    res += "]";
    return res;
}