#ifndef C_GARBAGE_COLLECTOR_HPP
#define C_GARBAGE_COLLECTOR_HPP

#include <memory>
#include <unordered_map>

template <typename T>
struct GarbageCollector
{
    GarbageCollector(GarbageCollector &) = delete;
    void operator=(const GarbageCollector &) = delete;
    GarbageCollector(GarbageCollector &&) = delete;
    void operator=(const GarbageCollector &&) = delete;

    static GarbageCollector<T> &get_instance()
    {
        static GarbageCollector<T> instance;
        return instance;
    }

    void dec_counter(T *val)
    {
        auto it = this->memory.find(val);
        if (it != this->memory.end())
        {
            it->second -= 1;
            if (it->second == 0)
            {
                this->memory.erase(val);
                delete it->first;
            }
        }
    }
    void inc_counter(T *val)
    {
        auto it = this->memory.find(val);
        if (it != this->memory.end())
        {
            it->second += 1;
        }
        else
        {
            this->memory.insert({val, 1});
        }
    }
    uint32_t get_counter(T *val)
    {
        auto it = this->memory.find(val);
        if (it != this->memory.end())
        {
            return it->second;
        }
        else
        {
            return 0;
        }
    }

private:
    GarbageCollector() = default;
    std::unordered_map<T *, uint32_t> memory;
};

#endif