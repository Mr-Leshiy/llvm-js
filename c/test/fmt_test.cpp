#include <gtest/gtest.h>
#include "fmt/fmt.hpp"

TEST(Fmt, Basic_test)
{
    EXPECT_EQ(format("{}, {}, {}", "hello", "world", 5),
              "hello, world, 5");

    EXPECT_EQ(format("{{{}, {}, {}}}", "hello", "world", 5),
              "{hello, world, 5}");

    EXPECT_EQ(format("{{{{{}}}, {}, {}}}", "hello", "world", 5),
              "{{hello}, world, 5}");
}

TEST(Fmt, invalid_format)
{
    EXPECT_NO_THROW(format("digit value: {}"));
}