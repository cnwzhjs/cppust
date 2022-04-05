/**
 * @file option.hpp
 * @author Tony Huang (cnwzhjs@gmail.com)
 * @brief A C++ implementation of Rust's `std::option::Option<T>` Type
 * @version 0.1
 * @date 2022-04-02
 * 
 * @copyright Copyright (c) 2022 Tony Huang
 */

#pragma once

#include "utils.hpp"

#include <optional>
#include <functional>
#include <iostream>

namespace cppust {

    template < typename T >
    class Option {
    private:
        enum class _Tag {
            None,
            Some,
        };

        union alignas(alignof(T)) _Union {
            T some_val;

            _Union() {}
            ~_Union() {}
        };

    public:
        ~Option() {
            deinit_union_();
        }
    
    public:
        Option(const Option& rhs): Option(rhs.tag_, rhs.union_) {}
        Option(Option&& rhs): Option(rhs.tag_, std::move(rhs.union_)) {}
        
        Option& operator=(const Option& rhs) {
            if (this == &rhs) { return *this; }

            if (tag_ == rhs.tag_) {
                switch (tag_) {
                    case _Tag::None:
                        break;
                    case _Tag::Some:
                        union_.some_val = rhs.union_.some_val;
                        break;
                }
            } else {
                deinit_union_();
                tag_ = rhs.tag_;
                tagged_init_(rhs.tag_, rhs.union_);
            }

            return *this;
        }
        
        Option& operator=(Option&& rhs) {
            if (this == &rhs) { return *this; }

            if (tag_ == rhs.tag_) {
                switch (tag_) {
                    case _Tag::None:
                        break;
                    case _Tag::Some:
                        union_.some_val = std::move(rhs.union_.some_val);
                        break;
                }
            } else {
                deinit_union_();
                tag_ = rhs.tag_;
                tagged_init_(rhs.tag_, std::move(rhs.union_));
            }

            return *this;
        }
    
    public:
        static Option None() {
            return Option(_Tag::None);
        }

        static Option Some(const T& val) {
            Option<T> output(_Tag::Some);
            new (&output.union_.some_val) T(val);
            return output;
        }

        static Option Some(T&& val) {
            Option<T> output(_Tag::Some);
            new (&output.union_.some_val) T(std::move(val));
            return output;
        }
    
    public:
        bool is_some() const { return tag_ == _Tag::Some; }
        bool is_none() const { return tag_ == _Tag::None; }

        T expect(const char* msg) const {
            if (tag_ == _Tag::None) {
                throw std::runtime_error(msg);
            } else {
                return union_.some_val;
            }
        }

        const T& expect_ref(const char* msg) const {
            if (tag_ == _Tag::None) {
                throw std::runtime_error(msg);
            } else {
                return union_.some_val;
            }
        }

        T& expect_ref(const char* msg) {
            if (tag_ == _Tag::None) {
                throw std::runtime_error(msg);
            } else {
                return union_.some_val;
            }
        }

        T unwrap() const {
            return expect("unwrapping Option::None");
        }

        const T& unwrap_ref() const {
            return expect_ref("unwrapping Option::None");
        }

        T& unwrap_ref() {
            return expect_ref("unwrapping Option::None");
        }

        T unwrap_or(const T& default_val) const {
            if (tag_ == _Tag::None) {
                return default_val;
            } else {
                return union_.some_val;
            }
        }

        template < typename U >
        Option<U> map(std::function<U(const T)> f) const {
            if (tag_ == _Tag::None) {
                return Option<U>::None();
            } else {
                return Option<U>::Some(f(union_.some_val));
            }
        }
    
    public:
        template < typename U >
        friend std::ostream& operator<<(std::ostream& os, const Option<U>& option);

        bool operator==(const Option<T>& rhs) const {
            if (this == &rhs) return true;

            if (tag_ != rhs.tag_) {
                return false;
            }

            switch (tag_) {
                case _Tag::Some:
                    return union_.some_val == rhs.union_.some_val;
                default:
                    return true;
            }
        }

        bool operator!=(const Option<T>& rhs) const {
            return !(*this == rhs);
        }

    private:
        _Union union_;
        _Tag tag_;

        explicit Option(_Tag tag): tag_(tag) {}

        Option(_Tag tag, const _Union& val): tag_(tag) {
            tagged_init_(tag, val);
        }

        Option(_Tag tag, _Union&& val): tag_(tag) {
            tagged_init_(tag, std::move(val));
        }

        void tagged_init_(_Tag tag, const _Union& union_val) {
            switch (tag) {
                case _Tag::None:
                    break;
                case _Tag::Some:
                    new (&union_.some_val) T(union_val.some_val);
                    break;
            }
        }

        void tagged_init_(_Tag tag, _Union&& union_val) {
            switch (tag) {
                case _Tag::None:
                    break;
                case _Tag::Some:
                    new (&union_.some_val) T(std::move(union_val.some_val));
                    break;
            }
        }

        void deinit_union_() {
            switch (tag_) {
                case _Tag::None:
                    break;
                case _Tag::Some:
                    union_.some_val.~T();
                    break;
            }
        }
    };

    template < typename T >
    std::ostream& operator<<(std::ostream& os, const cppust::Option<T>& option) {
        switch (option.tag_) {
            case cppust::Option<T>::_Tag::None:
                return os<<"None";
            case cppust::Option<T>::_Tag::Some:
                return os<<"Some("<<option.union_.some_val<<")";
            default:
                return os<<"<?>("<<int(option.tag_)<<")";
        }
    }

}
