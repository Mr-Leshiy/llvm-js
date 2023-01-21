#include "object.hpp"
#include "variable/variable.hpp"

Object::~Object()
{
    for (const auto &val : this->properties)
    {
        GarbageCollector<Variable>::get_instance().dec_counter(val.second);
    }
}

Object &Object::operator=(const Object &val)
{
    this->properties = val.properties;
    for (const auto &val : this->properties)
    {
        GarbageCollector<Variable>::get_instance().inc_counter(val.second);
    }
    return *this;
}

void Object::add_property(const std::string &key, Variable *val)
{
    this->properties[key] = val;
}

Variable *Object::get_property(const std::string &key, bool allocate)
{
    auto it = this->properties.find(key);
    if (it != this->properties.end())
    {
        return it->second;
    }
    else
    {
        auto *ret = GarbageCollector<Variable>::get_instance().allocate();
        if (allocate)
        {
            this->add_property(key, ret);
        }
        return ret;
    }
}

void Object::remove_property(const std::string &key)
{
    this->properties.erase(key);
}

bool Object::empty() const
{
    return this->properties.empty();
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
        res += el.first + ": " + el.second->to_string() + ",";
    }
    res += "}";
    return res;
}
