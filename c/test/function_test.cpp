#include <gtest/gtest.h>

#include "function/function.hpp"

Variable *foo(Variable **args)
{
    return nullptr;
}

TEST(Function, Basic_test)
{
    Function func(foo, 0);
    func.call(nullptr);
}