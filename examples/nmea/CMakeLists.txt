project(nmea VERSION 0.1)

add_custom_command(
    OUTPUT ${CMAKE_CURRENT_SOURCE_DIR}/include/nmea/decoder_status.fmt.hpp
           ${CMAKE_CURRENT_SOURCE_DIR}/include/nmea/decoder_status.inc.hpp
           ${CMAKE_CURRENT_SOURCE_DIR}/src/decoder_status.gen.cpp
    COMMAND cppust-gen -i ${CMAKE_CURRENT_SOURCE_DIR}/src/decoder_status.rs
                       -I ${CMAKE_CURRENT_SOURCE_DIR}/include
                       -O ${CMAKE_CURRENT_SOURCE_DIR}/src
                       -n "nmea"
    DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/src/decoder_status.rs
    VERBATIM
)

add_executable(${PROJECT_NAME} src/main.cpp src/decoder_status.gen.cpp)
target_include_directories(${PROJECT_NAME} PRIVATE include)
target_link_libraries(${PROJECT_NAME} cppust)
