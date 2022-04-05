/**
 * @file result.hpp
 * @author Tony Huang (cnwzhjs@gmail.com)
 * @brief A C++ implementation of Rust's `std::result::Result<T, E>` Type
 * @version 0.1
 * @date 2022-04-02
 * 
 * @copyright Copyright (c) 2022 Tony Huang
 */

#pragma once

#include "utils.hpp"
#include "option.hpp"

#include <optional>
#include <functional>

namespace cppust {

    template < typename T, typename E >
    class Result {
    private:
        static constexpr const size_t _kUnionAlign = utils::align_of_union_value<T, E>();

        enum class _Tag {
            Ok,
            Err,
        };

        union alignas(_kUnionAlign) _Union {
            T ok_val;
            E err_val;

            _Union() {}
            ~_Union() {}
        };

    public:
        ~Result() {
            deinit_union_();
        }

    private:
        _Union union_;
        _Tag tag_;   // putting _Tag at the end could optimize memory usage when types requires alignment 

        Result(_Tag _Tag): tag_(_Tag) {}

        Result(_Tag _Tag, const _Union& union_val): tag_(_Tag) {
            tagged_init_(_Tag, union_val);
        }

        Result(_Tag _Tag, _Union&& union_val): tag_(_Tag) {
            tagged_init_(_Tag, std::move(union_val));
        }

        void tagged_init_(_Tag _Tag, const _Union& union_val) {
            switch (_Tag) {
                case _Tag::Ok:
                    new (&union_.ok_val) T(union_val.ok_val);
                    break;
                case _Tag::Err:
                    new (&union_.err_val) E(union_val.err_val);
                    break;
            }
        }

        void tagged_init_(_Tag _Tag, _Union&& union_val) {
            switch (_Tag) {
                case _Tag::Ok:
                    new (&union_.ok_val) T(std::move(union_val.ok_val));
                    break;
                case _Tag::Err:
                    new (&union_.err_val) E(std::move(union_val.err_val));
                    break;
            }
        }

        void deinit_union_() {
            switch (tag_) {
                case _Tag::Ok:
                    union_.ok_val.~T();
                    break;
                case _Tag::Err:
                    union_.err_val.~E();
                    break;
            }
        }

    public:
        Result(const Result& rhs): Result(rhs.tag_, rhs.union_) {}
        Result(Result&& rhs): Result(rhs.tag_, std::move(rhs.union_)) {}

        Result& operator=(const Result& rhs) {
            if (this == &rhs) { return *this; }

            if (tag_ == rhs.tag_) {
                switch (tag_) {
                    case _Tag::Ok:
                        union_.ok_val = rhs.union_.ok_val;
                        break;
                    case _Tag::Err:
                        union_.err_val = rhs.union_.err_val;
                        break;
                }
            } else {
                deinit_union_();
                tag_ = rhs.tag_;
                tagged_init_(rhs.tag_, rhs.union_);
            }

            return *this;
        }

        Result& operator=(Result&& rhs) {
            if (this == &rhs) { return *this; }

            if (tag_ == rhs.tag_) {
                switch (tag_) {
                    case _Tag::Ok:
                        union_.ok_val = std::move(rhs.union_.ok_val);
                        break;
                    case _Tag::Err:
                        union_.err_val = std::move(rhs.union_.err_val);
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
        static Result Ok(const T& ok_val) {
            Result output(_Tag::Ok);
            new (&output.union_.ok_val) T(ok_val);
            return output;
        }

        static Result Err(const E& err_val) {
            Result output(_Tag::Err);
            new (&output.union_.err_val) E(err_val);
            return output;
        }

    public:
        CPPUST_DEFINE_TAG(ok, Ok, T)
        CPPUST_DEFINE_TAG(err, Err, E)

    public:
        template < typename U >
        Result<U, E> map(std::function<U(const T&)> f) const {
            if (tag_ == _Tag::Ok) {
                return Result<U, E>::ok(std::move(f(union_.ok_val)));
            } else {
                return Result<U, E>::err(union_.err_val);
            }
        }

        template < typename U >
        Result<U, E> map(std::function<U(T&&)> f) {
            if (tag_ == _Tag::Ok) {
                return Result<U, E>::ok(std::move(f(std::move(union_.ok_val))));
            } else {
                return Result<U, E>::err(union_.err_val);
            }
        }
    
    public:
        template < typename U, typename V >
        friend std::ostream& operator<<(std::ostream& os, const Result<U, V>&);

        bool operator==(const Result<T, E>& rhs) const {
            if (this == &rhs) return true;

            if (tag_ != rhs.tag_) {
                return false;
            }

            switch (tag_) {
                case _Tag::Ok:
                    return union_.ok_val == rhs.union_.ok_val;
                case _Tag::Err:
                    return union_.err_val == rhs.union_.err_val;
                default:
                    return true;
            }
        }

        bool operator!=(const Result<T, E>& rhs) const {
            return !(*this == rhs);
        }
    };

    template < typename T, typename E >
    std::ostream& operator<<(std::ostream& os, const Result<T, E>& result) {
        switch (result.tag_) {
            case Result<T, E>::_Tag::Ok:
                return os<<"Ok("<<result.union_.ok_val<<")";
            case Result<T, E>::_Tag::Err:
                return os<<"Err("<<result.union_.err_val<<")";
            default:
                return os<<"<?>("<<int(result.tag_)<<")";
        }
    }

}
