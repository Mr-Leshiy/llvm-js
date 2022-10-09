#include "object.hpp"
#include "variable.hpp"

void Object::add_property(std::string key, Variable *val)
{
    this->properties[key] = val;
}

Variable *Object::get_property(std::string key)
{
    auto it = this->properties.find(key);
    if (it != this->properties.end())
    {
        return it->second;
    }
    else
    {
        auto *ret = allocate();
        set_undefined(ret);
        return ret;
    }
}

void Object::remove_property(std::string key)
{
    this->properties.erase(key);
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
        res += " " + el.first + ": " + el.second->to_string() + " ";
    }
    res += "}";
    return res;
}