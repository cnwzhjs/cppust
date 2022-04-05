cmake_minimum_required(VERSION 3.5)

project(cppust-all LANGUAGES NONE VERSION 0.1)

enable_language(C)
enable_language(CXX)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED True)

add_subdirectory(cppust)
add_subdirectory(examples)
