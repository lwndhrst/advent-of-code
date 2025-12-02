cmake_minimum_required (VERSION 4.0)

project("aoc-2025")

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Enables clangd to find standard headers
if(CMAKE_EXPORT_COMPILE_COMMANDS)
  set(CMAKE_CXX_STANDARD_INCLUDE_DIRECTORIES ${CMAKE_CXX_IMPLICIT_INCLUDE_DIRECTORIES})
endif()

add_executable(day-01 src/01/main.cpp)
add_executable(day-02 src/02/main.cpp)
