#ifndef C_OBJECT_HPP
#define C_OBJECT_HPP

#include <string>
#include <unordered_map>

extern "C"
{
#include "variable.h"
}

struct Object
{
    void add_property(std::string key, Variable *val);
    Variable* get_property(std::string key);
    std::string to_string() const;

private:
    std::unordered_map<std::string, Variable *> properties;
};

#endif