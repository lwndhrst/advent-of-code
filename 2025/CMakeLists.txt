cmake_minimum_required (VERSION 4.0)

project("aoc-2025")

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Enables clangd to find standard headers
if(CMAKE_EXPORT_COMPILE_COMMANDS)
  set(CMAKE_CXX_STANDARD_INCLUDE_DIRECTORIES ${CMAKE_CXX_IMPLICIT_INCLUDE_DIRECTORIES})
endif()

find_package(Z3 REQUIRED)

add_executable(day-01 src/01/main.cpp)
add_executable(day-02 src/02/main.cpp)
add_executable(day-03 src/03/main.cpp)
add_executable(day-04 src/04/main.cpp)
add_executable(day-05 src/05/main.cpp)
add_executable(day-06 src/06/main.cpp)
add_executable(day-07 src/07/main.cpp)
add_executable(day-08 src/08/main.cpp)
add_executable(day-09 src/09/main.cpp)
add_executable(day-10 src/10/main.cpp)
add_executable(day-11 src/11/main.cpp)
add_executable(day-12 src/12/main.cpp)

target_include_directories(day-10 PRIVATE ${Z3_CXX_INCLUDE_DIRS})
target_link_libraries(day-10 PRIVATE ${Z3_LIBRARIES})
target_compile_options(day-10 PRIVATE ${Z3_COMPONENT_CXX_FLAGS})
