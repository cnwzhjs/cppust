/**
 * @file markers.hpp
 * @author Tony Huang (cnwzhjs@gmail.com)
 * @brief Markers like `Send`, `Sync`
 * @version 0.1
 * @date 2022-04-04
 * 
 * @copyright Copyright (c) 2022 Tony Huang
 */

#pragma once

#include "types.hpp"

#include <memory>

namespace cppust {

    template < typename T >
    struct is_send {
        static constexpr const bool value = false;
    };

    template < typename T >
    struct is_sync {
        static constexpr const bool value = false;
    };

    template < typename T >
    struct is_send<std::unique_ptr<T>> {
        static constexpr const bool value = is_send<T>::value;
    };

    template < typename T >
    struct is_send<std::shared_ptr<T>> {
        static constexpr const bool value = is_send<T>::value || is_sync<T>::value;
    };

#if __cplusplus > 201703L
    template < typename T >
    inline constexpr bool is_send_v = is_send<T>::value;

    tempalte < typename T >
    inline constexpr bool is_sync_v = is_sync<T>::value;
#endif

}

#define CPPUST_IMPL_SEND_FOR(Type) \
    template<> struct ::cppust::is_send<Type> { \
        static constexpr const bool value = true; \
    };

#define CPPUST_IMPL_SYNC_FOR(Type) \
    template<> struct ::cppust::is_sync<Type> { \
        static constexpr const bool value = true; \
    };
