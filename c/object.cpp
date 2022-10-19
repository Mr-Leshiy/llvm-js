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

Variable *Object::get_property(const Variable &key)
{
    return this->get_property(key.to_string());
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
        res += el.first + ": " + el.second->to_string() + ",";
    }
    res += "}";
    return res;
}

void init_object(Variable *self)
{
    assert(self != nullptr);

    self->flag = Type::Object;
}

void add_property(Variable *self, const char *key, Variable *val)
{
    assert(self != nullptr);

    // TODO print runtime error message
    if (self->flag == Type::Object)
    {
        self->object_field.add_property(key, val);
    }
}

Variable *get_property_by_str(Variable *self, const char *key)
{
    assert(self != nullptr);

    return self->object_field.get_property(key);
}

Variable *get_property_by_var(Variable *self, Variable *key)
{
    assert(self != nullptr);
    assert(key != nullptr);

    return self->object_field.get_property(*key);
}

void remove_property(Variable *self, const char *key)
{
    assert(self != nullptr);

    self->object_field.remove_property(key);
}
