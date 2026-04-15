cmake_minimum_required(VERSION 3.10)
project(emp)

set(CMAKE_CXX_STANDARD 17)

# Placeholder: useful if integrating native modules later
add_custom_target(emp_rust ALL
    COMMAND cargo build
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}/build
)
