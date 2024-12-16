# See LICENSE file for license information.
cmake_minimum_required(VERSION 3.10)
project(
    forestry 
    VERSION 1.0.0
    DESCRIPTION "A simple, fast, and efficient logging library for C.")

# Add shared library
add_library(
    ${PROJECT_NAME}
    SHARED src/forestry.c)
set_property(
    TARGET ${PROJECT_NAME}
    PROPERTY POSITION_INDEPENDENT_CODE ON)
target_include_directories(
    ${PROJECT_NAME}
    PUBLIC include)

# Add static library
add_library(
    ${PROJECT_NAME}_static
    STATIC src/forestry.c)
target_include_directories(
    ${PROJECT_NAME}_static
    PUBLIC include)

# Add test executable
add_executable(
    test
    src/test.c)
target_include_directories(
    test
    PRIVATE include)
target_link_libraries(
    test
    ${PROJECT_NAME}_static)

# Library installation rules
install(
    TARGETS ${PROJECT_NAME} ${PROJECT_NAME}_static
    RUNTIME DESTINATION ${CMAKE_INSTALL_PREFIX}/bin
    LIBRARY DESTINATION ${CMAKE_INSTALL_PREFIX}/lib
    ARCHIVE DESTINATION ${CMAKE_INSTALL_PREFIX}/lib/${PROJECT_NAME})

# Header installation rules
install(
    FILES include/forestry.h
    DESTINATION ${CMAKE_INSTALL_PREFIX}/include)
