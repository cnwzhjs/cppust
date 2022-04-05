# cppust

A toy project trying to port some of Rust's features to C++

> The cpprust library and generated code requires at least C++11, cos we use constexpr, variadic tempaltes heavily.
>
> C++14 and above is recommended.

## License

This project is licensed under [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0).

## Components

1. `cppust` header only library

    A header only library to provide some utility functions, `std::result::Result<T, E>` and `std::option::Option<T>` in rust.

2. `examples` examples for `cppust`

## Usage

### Primitive Types

1. Signed Integers: `i8`, `i16`, `i32`, `i64`
2. Unsigned Integers: `u8`, `u16`, `u32`, `u64`
3. Size Types: `usize`, `isize`

```cpp
#include <cppust/cppust.hpp>

struct SomeData {
    cppust::u8 u8_field;
    cppust::usize usize_field;
};
```

### Standard Library Features

1. `std::option::Option<T>`

    ```cpp
    #include <cppust/cppust.hpp>

    using namespace cppust;

    void use_option() {
        // Construct options
        auto none = Option<int>::None();
        auto some_int = Option<int>::Some(32);

        // Compare options
        assert(none != some_int);

        // Print options
        std::cout<<"some_int: "<<some_int<<"\n";

        // Option map
        auto is_even = some_int.map<bool>([](int v) {
            return v % 2 == 0;
        });
    }
    ```

2. `std::result::Result<T, E>`

    ```cpp
    #include <cppust/cppust.hpp>

    // Define your error type
    enum Error {
        OperationFailed,
        OperationTimeout,
        Unsupported,
    };

    static std::ostream& operator<<(std::ostream& os, const Error& err) {
        switch (err) {
        case Error::OperationFailed:
            return os<<"OperationFailed";
        case Error::OperationTimeout:
            return os<<"OperationTimeout";
        case Error::Unsupported:
            return os<<"Unsupported";
        default:
            return os<<"Invalid("<<int(err)<<")";
        }
    }

    // Define your result type
    template <typename T>
    using Result = cppust::Result<T, Error>;

    Result<std::string> do_some_operation() {
        return Result<std::string>::Ok("hello, world");
    }

    void use_result() {
        auto res = do_some_operation();

        std::cout<<"Operation result: "<<res<<"\n";
        std::cout<<"Operation failed: "<<Result<std::string>::Err(Error::Unsupported)<<"\n";
        std::cout<<"Result unwrap: "<<res.unwrap()<<"\n";
    }
    ```

3. Markers

    Marker traits in Rust are very similar to C++'s type traits. `Send` and `Sync` are almost the most important markers.

    In cppust, I defined two type traits for this use:

    ```cpp
    #include <cppust/cppust.hpp>

    class SomePowerfulClass {};

    CPPUST_IMPL_SEND_FOR(SomePowerfulClass)

    void use_markers() {
        std::cout<<"SomePowerfulClass is_send: "<<cppust::is_send<SomePowerfulClass>::value<<"\n";
        std::cout<<"SomePowerfulClass is_sync: "<<cppust::is_sync<SomePowerfulClass>::value<<"\n";
        assert(cppust::is_send_v<SomePowerfulClass>);   // C++ 14 and above
    }
    ```

## Roadmap

### v0.2

1. Add extra methods to `Option<T>` and `Result<T, E>`
2. Improve `is_send<T>` and `is_sync<T>` marker improvements

### v0.3 to v0.5

1. Extra improvements on the library

### v0.6

1. Add `cppust-gen` code generator to covnert Rust enum types to C++ code

