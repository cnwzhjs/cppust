/**
 * @file types.hpp
 * @author Tony Huang (cnwzhjs@gmail.com)
 * @brief Number type alias
 * @version 0.1
 * @date 2022-04-04
 * 
 * @copyright Copyright (c) 2022 Tony Huang
 */

#pragma once

#include <stddef.h>
#include <unistd.h>

#include <cstdint>
#include <vector>
#include <tuple>

namespace cppust {

    using u8 = std::uint8_t;
    using u16 = std::uint16_t;
    using u32 = std::uint32_t;
    using u64 = std::uint64_t;

    using i8 = std::int8_t;
    using i16 = std::int16_t;
    using i32 = std::int32_t;
    using i64 = std::int64_t;

    using f32 = float;
    using f64 = double;

    using usize = size_t;
    using isize = ssize_t;

}
