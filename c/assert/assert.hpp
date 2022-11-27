#ifndef C_ASSERT_HPP
#define C_ASSERT_HPP

#include "fmt/fmt.hpp"

// executes only when compiled in DEBUG mode. similar to how <cassert> works
#if defined(NDEBUG)
#define ASSERT_MSG(x, ...)
#define ASSERT(x)
#else
#define ASSERT_MSG(x, ...)                                                       \
    /* GCOVR_EXCL_START */                                                       \
    if (!(x))                                                                    \
    {                                                                            \
        auto msg = fmt::format("Assertion failed at {}:{} inside {}:\n{}\n{}\n", \
                               __FILE__,                                         \
                               __LINE__,                                         \
                               __FUNCTION__,                                     \
                               #x,                                               \
                               fmt::sprintf(__VA_ARGS__));                       \
        /* print to stdout */ fmt::fprintf(stdout, msg);                         \
        /* print to stderr */ fmt::fprintf(stderr, msg);                         \
        /* die */ std::terminate();                                              \
    } /* GCOVR_EXCL_STOP */

#define ASSERT(x) ASSERT_MSG(x, " ");
#endif

#endif