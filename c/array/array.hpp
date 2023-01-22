#ifndef C_ARRAY_HPP
#define C_ARRAY_HPP

#include <stdint.h>
#include <vector>
#include <string>

struct Variable;
struct Number;

struct Array
{
    ~Array();
    Array() = default;
    Array(std::vector<Variable *> values) : values(values) {}
    Array &operator=(const Array &);

    void push(Variable &value);
    Variable *pop();
    void put(Variable &value, uint32_t index);
    void put(Variable &value, const Number &index);
    Variable *get(uint32_t index, bool allocate);
    Variable *get(const Number &index, bool allocate);
    uint32_t len() const;
    bool empty() const;
    void clear();

    std::string to_string() const;

    friend bool operator==(const Array &a, const Array &b);
    friend bool operator!=(const Array &a, const Array &b);

private:
    std::vector<Variable *> values;
};

#endif