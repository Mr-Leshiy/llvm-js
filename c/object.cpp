#include "object.hpp"
#include "variable.hpp"

void add_property(Object &self, std::string key, Variable *val)
{
    self.properties[key] = val;
}

Variable *get_property(Object &self, std::string key)
{
    auto it = self.properties.find(key);
    if (it != self.properties.end())
    {
        return it->second;
    }
    else
    {
        auto *ret = variable_allocate();
        set_undefined(ret);
        return ret;
    }
}

void remove_property(Object &self, std::string key)
{
    self.properties.erase(key);
}

bool operator==(const Object &a, const Object &b)
{
    return a.properties == b.properties;
}

bool operator!=(const Object &a, const Object &b)
{
    return !(a == b);
}

std::string Object::to_string() const
{
    std::string res = "{";
    for (const auto &el : this->properties)
    {
        res += el.first + ": " + el.second->to_string();
    }
    res += "}";
    return res;
}