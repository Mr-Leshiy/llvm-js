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
    friend void add_property(Object &object, std::string key, Variable *val);
    friend Variable *get_property(Object &object, std::string key);
    friend void remove_property(Object &object, std::string key);
    std::string to_string() const;

    friend bool operator==(const Object &a, const Object &b);
    friend bool operator!=(const Object &a, const Object &b);

private:
    std::unordered_map<std::string, Variable *> properties;
};

#endif