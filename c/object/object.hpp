#ifndef C_OBJECT_HPP
#define C_OBJECT_HPP

#include <string>
#include <unordered_map>

struct Variable;

struct Object
{
    ~Object();
    Object() = default;
    Object &operator=(const Object &);

    void add_property(const std::string &key, Variable *val);
    Variable *get_property(const std::string &key, bool allocate);
    void remove_property(const std::string &key);
    bool empty() const;

    std::string to_string() const;

    friend bool operator==(const Object &a, const Object &b);
    friend bool operator!=(const Object &a, const Object &b);

private:
    std::unordered_map<std::string, Variable *> properties;
};

#endif