#ifndef C_FMT_HPP
#define C_FMT_HPP

#include <string.h>
#include <fmt/format.h>
#include <fmt/printf.h>

template <typename S, typename... Args>
inline std::string format(const S &format_str, Args &&...args)
{
    try
    {
        return fmt::format(format_str, (Args &&) args...);
    }
    catch (const fmt::format_error &)
    {
        fmt::printf("invalid string formatting, str: %s\n", format_str);
    }
    return "";
}

#endif