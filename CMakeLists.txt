cmake_minimum_required(VERSION 3.10)
project(
    forestry 
    VERSION 0.1.0
    DESCRIPTION "A simple, fast, and efficient logging library for C.")
# Constants
set(LIB_SRC src/forestry.c)
set(TEST_SRC src/test.c)
set(FORESTRY_INCLUDE include)

# Add shared library
add_library(
    ${PROJECT_NAME}
    SHARED ${LIB_SRC})
# Force position-independent code for shared lib (i.e. "-fPIC")
set_property(
    TARGET ${PROJECT_NAME}
    PROPERTY POSITION_INDEPENDENT_CODE ON)
target_include_directories(
    ${PROJECT_NAME}
    PUBLIC ${FORESTRY_INCLUDE})

# Add static library
add_library(
    ${PROJECT_NAME}_static
    STATIC ${LIB_SRC})
target_include_directories(
    ${PROJECT_NAME}_static
    PUBLIC ${FORESTRY_INCLUDE})

# Add test executable
add_executable(
    ${PROJECT_NAME}_test
    ${TEST_SRC})
target_include_directories(
    ${PROJECT_NAME}_test
    PRIVATE ${FORESTRY_INCLUDE})
# Link static lib to test executable
target_link_libraries(
    ${PROJECT_NAME}_test
    ${PROJECT_NAME}_static)

# Install rules
# Shared library
install(
    TARGETS ${PROJECT_NAME}
    RUNTIME DESTINATION bin
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib/${PROJECT_NAME})

# Static library
install(
    TARGETS ${PROJECT_NAME}_static
    DESTINATION lib/${PROJECT_NAME})

# Include
install(
    DIRECTORY ${FORESTRY_INCLUDE}
    DESTINATION include)
