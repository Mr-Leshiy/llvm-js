#ifndef C_GARBAGE_COLLECTOR_HPP
#define C_GARBAGE_COLLECTOR_HPP

#include <memory>
#include <string>
#include <unordered_map>

template <typename T>
struct GarbageCollector;

template <typename T>
struct Allocator
{

    friend GarbageCollector<T>;

private:
    static T *allocate()
    {
        return T::allocate_impl();
    }
};

template <typename T>
struct GarbageCollector
{
    ~GarbageCollector()
    {
        for (const auto &el : this->memory)
        {
            delete el.first;
        }
    }

    GarbageCollector(GarbageCollector &) = delete;
    void operator=(const GarbageCollector &) = delete;
    GarbageCollector(GarbageCollector &&) = delete;
    void operator=(const GarbageCollector &&) = delete;

    static GarbageCollector<T> &get_instance()
    {
        static GarbageCollector<T> instance;
        return instance;
    }

    T *allocate()
    {
        auto val = Allocator<T>::allocate();
        this->memory.insert({val, 1});
        return val;
    }

    void dec_counter(const T *val)
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
        // printf("GB: \n %s\n", this->to_string().c_str());
    }

    void inc_counter(const T *val)
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
        // printf("GB: \n %s\n", this->to_string().c_str());
    }

    uint32_t get_counter(const T *val)
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

    uint32_t get_variables_count()
    {
        return this->memory.size();
    }

    std::string to_string() const
    {
        std::string res = "";
        for (const auto &el : this->memory)
        {
            res += "[ address: " + std::to_string(((uint64_t)el.first)) + " counter: " + std::to_string(el.second) + "]\n";
        }

        return res;
    }

private:
    GarbageCollector() = default;
    std::unordered_map<const T *, uint32_t> memory;
};

#endif