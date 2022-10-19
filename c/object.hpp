#ifndef C_OBJECT_HPP
#define C_OBJECT_HPP

#include <string>
#include <unordered_map>

extern "C"
{
    #include "object.h"
}

struct Object
{
    void add_property(std::string key, Variable *val);
    Variable *get_property(std::string key);
    Variable *get_property(const Variable &key);
    void remove_property(std::string key);

    std::string to_string() const;

    friend bool operator==(const Object &a, const Object &b);
    friend bool operator!=(const Object &a, const Object &b);

private:
    std::unordered_map<std::string, Variable *> properties;
};

#endif