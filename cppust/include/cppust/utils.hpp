/**
 * @file util.h
 * @author Tony Huang (cnwzhjs@gmail.com)
 * @brief Some utilities used by cppust
 * @version 0.1
 * @date 2022-04-02
 * 
 * @copyright Copyright (c) 2022 Tony Huang
 */

#pragma once

#include <stddef.h>

namespace cppust { namespace utils {

    namespace {
        constexpr size_t gcd(size_t a, size_t b) {
            return (b == 0) ? a : gcd(b, a % b);
        }

        constexpr size_t lcm(size_t a, size_t b) {
            return a * b / gcd(a, b);
        }

        template < typename... Types >
        struct align_of_union_value_t;

        template < typename T, typename... TRest >
        struct align_of_union_value_t<T, TRest...> {
            static constexpr const size_t value = lcm(alignof(T), align_of_union_value_t<TRest...>::value);
        };

        template < typename T >
        struct align_of_union_value_t<T> {
            static constexpr const size_t value = alignof(T);
        };
    }

    template < typename... Types >
    constexpr size_t align_of_union_value() {
        return align_of_union_value_t<Types...>::value;
    }

} }

#define CPPUST_DEFINE_REF_UNCHECK_CONST(name, tag, type) \
    const type& name ## _ref_uncheck() const { \
        assert(tag_ == _Tag::tag); \
        return union_.name ## _val; \
    }

#define CPPUST_DEFINE_REF_UNCHECK_MUT(name, tag, type) \
    type& name ## _ref_uncheck() { \
        assert(tag_ == _Tag::tag); \
        return union_.name ## _val; \
    }

#define CPPUST_DEFINE_REF_UNCHECK(name, tag, type) \
    CPPUST_DEFINE_REF_UNCHECK_CONST(name, tag, type) \
    CPPUST_DEFINE_REF_UNCHECK_MUT(name, tag, type)

#define CPPUST_DEFINE_REF_CONST(name, tag, type) \
    const type& name ## _ref() const { \
        if (tag_ != _Tag::tag) { throw std::runtime_error("tag is not " #tag); } \
        return union_.name ## _val; \
    }

#define CPPUST_DEFINE_REF_MUT(name, tag, type) \
    type& name ## _ref() { \
        if (tag_ != _Tag::tag) { throw std::runtime_error("tag is not " #tag); } \
        return union_.name ## _val; \
    }

#define CPPUST_DEFINE_REF(name, tag, type) \
    CPPUST_DEFINE_REF_CONST(name, tag, type) \
    CPPUST_DEFINE_REF_MUT(name, tag, type)

#define CPPUST_DEFINE_PTR_CONST(name, tag, type) \
    const type* name ## _ptr() const { \
        return (tag_ == _Tag::tag) ? union_.name ## _val : nullptr; \
    }

#define CPPUST_DEFINE_PTR_MUT(name, tag, type) \
    type* name ## _ptr() { \
        return (tag_ == _Tag::tag) ? union_.name ## _val : nullptr; \
    }

#define CPPUST_DEFINE_PTR(name, tag, type) \
    CPPUST_DEFINE_PTR_CONST(name, tag, type) \
    CPPUST_DEFINE_PTR_MUT(name, tag, type)

#define CPPUST_DEFINE_ACCESSORS(name, tag, type) \
    CPPUST_DEFINE_REF_UNCHECK(name, tag, type) \
    CPPUST_DEFINE_REF(name, tag, type) \
    CPPUST_DEFINE_PTR(name, tag, type)

#define CPPUST_DEFINE_IS(name, tag) \
    bool is_ ## name() const { return tag_ == _Tag::tag; }

#define CPPUST_DEFINE_TAG(name, tag, type) \
    CPPUST_DEFINE_ACCESSORS(name, tag, type) \
    CPPUST_DEFINE_IS(name, tag)
