/**
 * @file fmt.hpp
 * @author Tony Huang (cnwzhjs@gmail.com)
 * @brief `std::fmt::Display` and `std::fmt::Debug` ports
 * @version 0.1
 * @date 2022-04-06
 * 
 * @copyright Copyright (c) 2022 Tony Huang
 */

#pragma once

#include "types.hpp"

#include <iostream>
#include <vector>
#include <tuple>

namespace cppust {

    template < typename T >
    struct debug {
        static std::ostream& fmt(const T& self, std::ostream& os) {
            return os << self;
        }
    };

    template < typename T >
    struct as_debug_t {
        const T& to_debug;

        explicit as_debug_t(const T& to_debug)
            : to_debug(to_debug)
        {}
    };

    template < typename T >
    as_debug_t<T> as_debug(const T& to_debug) {
        return as_debug_t<T>(to_debug);
    }

    template < typename T >
    std::ostream& operator<<(std::ostream& os, const as_debug_t<T>& self) {
        return debug<T>::fmt(self.to_debug, os);
    }

    template <>
    struct debug<std::vector<u8>> {
        static std::ostream& fmt(const std::vector<u8>& self, std::ostream& os) {
            os<<"<";
            for (size_t i = 0; i < self.size(); i++) {
                if (i != 0) {
                    os<<" ";
                }

                os<<std::hex<<(int)self[i] / 16<<std::hex<<(int)self[i] % 16;
            }
            return os<<">";
        }
    };

    template <typename T>
    struct debug<std::vector<T>> {
        static std::ostream& fmt(const std::vector<T>& self, std::ostream& os) {
            os<<"<";
            for (size_t i = 0; i < self.size(); i++) {
                if (i != 0) {
                    os<<",";
                }

                os<<as_debug(self[i]);
            }
            return os<<">";
        }
    };

    template <typename...Args>
    struct debug<std::tuple<Args...>> {
        static std::ostream& fmt(const std::tuple<Args...>& self, std::ostream& os) {
            os<<"(";
            foreach_fmt<sizeof...(Args) - 1>::fmt(self, os);
            return os<<")";
        }

    private:
        template < size_t N >
        struct foreach_fmt {
            static std::ostream& fmt(const std::tuple<Args...>& self, std::ostream& os) {
                return foreach_fmt<N-1>::fmt(self, os)<<","<<as_debug(std::get<N>(self));
            }
        };

        template <>
        struct foreach_fmt<0> {
            static std::ostream& fmt(const std::tuple<Args...>& self, std::ostream& os) {
                return os<<as_debug(std::get<0>(self));
            }
        };
    };

}
