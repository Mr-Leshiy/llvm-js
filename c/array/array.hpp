#ifndef C_ARRAY_HPP
#define C_ARRAY_HPP

#include <vector>
#include <string>

struct Variable;
struct Number;

struct Array
{
    Array() = default;
    Array(std::vector<Variable *> values) : values(values) {}

    void push(Variable &value);
    Variable *pop();
    void put(Variable &value, uint32_t index);
    void put(Variable &value, const Number &index);
    Variable *get(uint32_t index);
    Variable *get(const Number &index);
    uint32_t len() const;

    std::string to_string() const;

private:
    std::vector<Variable *> values;
};

#endif