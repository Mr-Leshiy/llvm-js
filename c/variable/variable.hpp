#ifndef C_VARIABLE_HPP
#define C_VARIABLE_HPP

#include <string>

#include "object/object.hpp"
#include "number/number.hpp"
#include "array/array.hpp"
#include "garbage_collector/garbage_collector.hpp"

enum class Type
{
    Undefined,
    Null,
    Number,
    Boolean,
    String,
    Object,
    Array,
};

namespace test
{
    struct VariableTest;
}

struct Variable
{
    Variable &operator=(const Variable &);

    Variable &set_undefined();
    Variable &set_null();
    Variable &set_number(const Number &);
    Variable &set_boolean(bool);
    Variable &set_string(const std::string &);
    Variable &set_object(const Object &);
    Variable &set_array(const Array &);

    Type get_flag() const;
    bool get_boolean() const;
    const std::string &get_string() const;
    const Number &get_number() const;
    const Object &get_object() const;
    const Array &get_array() const;

    bool to_boolean() const;
    Number to_number() const;
    std::string to_string() const;

    void add_property(const std::string &, Variable *);
    void add_property(const Variable &, Variable *);
    Variable *get_property(const std::string &);
    Variable *get_property(const Variable &);
    void remove_property(const std::string &);

    friend Variable operator+(const Variable &a, const Variable &b);
    friend Variable operator-(const Variable &a, const Variable &b);
    friend Variable operator*(const Variable &a, const Variable &b);
    friend Variable operator/(const Variable &a, const Variable &b);

    friend bool operator==(const Variable &a, const Variable &b);
    friend bool operator!=(const Variable &a, const Variable &b);
    friend bool operator>(const Variable &a, const Variable &b);
    friend bool operator>=(const Variable &a, const Variable &b);
    friend bool operator<(const Variable &a, const Variable &b);
    friend bool operator<=(const Variable &a, const Variable &b);

    friend bool operator!(const Variable &a);
    friend Variable operator&&(const Variable &a, const Variable &b);
    friend Variable operator||(const Variable &a, const Variable &b);

    // Allocator impl
    friend Allocator<Variable>;
    // for tests
    friend test::VariableTest;

private:
    static Variable *allocate_impl()
    {
        return new Variable();
    }

    Variable();

    Type flag;
    Number number_field;
    bool boolean_field;
    std::string string_field;
    Object object_field;
    Array array_field;
};

namespace test
{
    struct VariableTest : Variable
    {
        VariableTest()
        {
            this->set_undefined();
        }
    };
}

#endif