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

    return allocate();
}

Variable *Array::get(Variable &index)
{
    index = *convert_to_number(&index);
    if (index.flag == Type::Number)
    {
        if ((uint32_t)index.number_field < this->len())
        {
            return this->values[(uint32_t)index.number_field];
        }
    }

    return allocate();
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