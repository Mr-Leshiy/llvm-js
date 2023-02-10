#ifndef C_GARBAGE_COLLECTOR_HPP
#define C_GARBAGE_COLLECTOR_HPP

#include <memory>
#include <string>
#include <unordered_map>

template <typename T>
struct GarbageCollector;

struct Counter
{
    Counter(uint32_t counter) : counter(counter) {}

    void inc()
    {
        this->counter += 1;
    }

    void dec()
    {
        this->counter -= 1;
    }

    uint32_t get_counter() const
    {
        return this->counter;
    }

private:
    uint32_t counter;
};

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
        // printf("~GarbageCollector: \n %s", this->to_string().c_str());
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
        this->memory.insert({val, Counter(1)});
        return val;
    }

    void dec_counter(const T *val)
    {
        auto it = this->memory.find(val);
        if (it != this->memory.end())
        {
            it->second.dec();
            if (it->second.get_counter() == 0)
            {
                delete it->first;
                this->memory.erase(val);
            }
        }
    }

    void inc_counter(const T *val)
    {
        auto it = this->memory.find(val);
        if (it != this->memory.end())
        {
            it->second.inc();
        }
        else
        {
            this->memory.insert({val, Counter(1)});
        }
    }

    Counter get_counter(const T *val)
    {
        auto it = this->memory.find(val);
        if (it != this->memory.end())
        {
            return it->second;
        }
        else
        {
            return Counter(0);
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
            res += "[ address: " + std::to_string(((uint64_t)el.first)) + " counter: " + std::to_string(el.second) + " data: " + el.first->to_string() + "]\n";
        }

        return res;
    }

private:
    GarbageCollector() = default;
    std::unordered_map<const T *, Counter> memory;
};

#endif