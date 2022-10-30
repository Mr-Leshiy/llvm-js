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
        return new Variable();
    }

    auto *ret = this->values.back();
    this->values.pop_back();
    return ret;
}

void Array::put(Variable &value, uint32_t index)
{
    if (index < this->len())
    {
        this->values[index] = &value;
    }
    else
    {
        while (index > this->len())
        {
            this->push(*(new Variable()));
        }
        this->push(value);
    }
}

void Array::put(Variable &value, const Number &index)
{
    if (index.get_type() == NumberType::Number)
    {
        double i = index.get_value();
        if (i >= 0)
        {
            this->put(value, index.get_value());
        }
    }
}

Variable *Array::get(uint32_t index, bool allocate)
{
    if (index < this->len())
    {
        return this->values[index];
    }
    else
    {
        auto *ret = new Variable();
        if (allocate)
        {
            this->put(*ret, index);
        }
        return ret;
    }
}

Variable *Array::get(const Number &index, bool allocate)
{
    if (index.get_type() == NumberType::Number)
    {
        return this->get(index.get_value(), allocate);
    }
    else
    {
        return new Variable();
    }
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