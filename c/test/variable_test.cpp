#include <gtest/gtest.h>

extern "C" {
#include "variable.h"
}

TEST(VariableType, Basic_test)
{
    VariableType* var = allocate();
    EXPECT_TRUE(true);
}