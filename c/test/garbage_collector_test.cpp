#include <gtest/gtest.h>

#include "garbage_collector/garbage_collector.hpp"

TEST(GarbageCollector, Basic_test)
{
    auto &gb = GarbageCollector<uint32_t>::get_instance();

    auto *val1 = new uint32_t(110);
    EXPECT_EQ(*val1, 110);
    EXPECT_EQ(gb.get_counter(val1).get_counter(), 0);

    gb.inc_counter(val1);
    EXPECT_EQ(gb.get_counter(val1).get_counter(), 1);

    gb.inc_counter(val1);
    EXPECT_EQ(gb.get_counter(val1).get_counter(), 2);

    gb.dec_counter(val1);
    EXPECT_EQ(gb.get_counter(val1).get_counter(), 1);

    gb.dec_counter(val1);
    EXPECT_EQ(gb.get_counter(val1).get_counter(), 0);
}