#ifndef C_ARRAY_HPP
#define C_ARRAY_HPP

#include <vector>
#include <string>

extern "C"
{
#include "variable.h"
}

struct Array
{
    Array() = default;
    Array(std::vector<Variable *> values) : values(values) {}
    void push(Variable &value);
    Variable *pop();
    Variable *get(uint32_t index);
    Variable *get(Variable &index);
    uint32_t len() const;
    std::string to_string() const;

private:
    std::vector<Variable *> values;
};

#endif