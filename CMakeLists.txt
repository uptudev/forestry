cmake_minimum_required(VERSION 3.10)
project(forestry 
    VERSION 0.1.0
    DESCRIPTION "A simple, fast, and efficient logging library for C.")
set(libsrc src/forestry.c)
set(testsrc src/test.c)
add_library(forestry SHARED ${libsrc})
target_include_directories(forestry PUBLIC include)
add_library(forestry_static STATIC ${libsrc})
target_include_directories(forestry_static PUBLIC include)
add_executable(forestry_test ${testsrc})
target_include_directories(forestry_test PRIVATE include)
target_link_libraries(forestry_test forestry_static)
