/**
 * @file main.cpp
 * @author Tony Huang (cnwzhjs@gmail.com)
 * @brief An simple example of how to use cppust
 * @version 0.1
 * @date 2022-04-02
 * 
 * @copyright Copyright (c) 2022 Tony Huang
 */

#include <cppust/cppust.hpp>

#include <iostream>

enum class NoDisplay {};

void use_option() {
    using namespace cppust;

    auto none = Option<int>::None();
    auto some_int = Option<int>::Some(32);
    auto none_no_display = Option<NoDisplay>::None();

    std::cout<<"none: "<<none<<"\n";
    std::cout<<"none == Option<int>::None(): "<<(none == Option<int>::None())<<"\n";
    std::cout<<"none != Option<int>::None(): "<<(none != Option<int>::None())<<"\n";
    std::cout<<"some_int: "<<some_int<<"\n";
    std::cout<<"some_int == Option<int>::None(): "<<(some_int == Option<int>::None())<<"\n";
    std::cout<<"some_int != Option<int>::None(): "<<(some_int != Option<int>::None())<<"\n";

    // the following code should introduce compile or link error
    // std::cout<<"none_no_display: "<<none_no_display<<"\n";

    auto some_bool = some_int.map<bool>([](int v) {
        return v % 2 == 0;
    });

    std::cout<<"some_bool: "<<some_bool<<"\n";
}

enum class Error {
    OperationFailed,
    OperationTimeout,
    OperationNotSupported,
};

std::ostream& operator<<(std::ostream& os, const Error& error) {
    switch (error) {
        case Error::OperationFailed:
            return os<<"OperationFailed";
        case Error::OperationTimeout:
            return os<<"OperationTimeout";
        case Error::OperationNotSupported:
            return os<<"OperationNotSupported";
        default:
            return os<<"Error("<<int(error)<<")";
    }
}

template < typename T >
using Result = cppust::Result<T, Error>;

void use_result() {
    auto ok = Result<std::string>::Ok("hello, world");
    auto err = Result<std::string>::Err(Error::OperationFailed);

    std::cout<<"ok: "<<ok<<"\n";
    std::cout<<"err: "<<err<<"\n";
}

CPPUST_IMPL_SYNC_FOR(Error)

void use_markers() {
    assert(!cppust::is_send<Error>::value);
    assert(cppust::is_sync<Error>::value);

    std::cout<<"Error is_send: "<<cppust::is_send<Error>::value
        <<", is_sync: "<<cppust::is_sync<Error>::value
        <<"\n";

    std::cout<<"std::shared_ptr<Error> is_send: "<<cppust::is_send<std::shared_ptr<Error>>::value
        <<", is_sync: "<<cppust::is_sync<std::shared_ptr<Error>>::value
        <<"\n";
}

int main() {
    use_option();
    use_result();
    use_markers();

    return 0;
}
