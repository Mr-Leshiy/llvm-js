#include <gtest/gtest.h>

#include "function/function.hpp"
#include "variable/variable.hpp"

Variable *foo(Variable **args)
{
    return args[0];
}

TEST(Function, Basic_test)
{
    Function func(foo, 1);

    auto val = func.call({});
    EXPECT_EQ(val->get_flag(), Type::Undefined);

    Variable arg = test::VariableTest().set_boolean(true);
    val = func.call({&arg});
    EXPECT_EQ(arg, *val);
}