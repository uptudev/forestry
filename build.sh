#!/bin/sh

# build.sh
# ^^^^^^^^
# This script is used to build the project.
# It does the following in sequence:
# 1.a) Cleans the build directory if it exists.
#   b) Creates a new build directory if it does not exist.
# 2. Changes the working directory to the build directory.
# 3. Runs cmake to generate the build files.
# 4. Runs make to build the project.
# 5. Deletes the build artifacts.
# 6. Hardlinks the header file to the build directory.
#
# If any of the steps fail, the script will print an error message and exit.
# The script will print a success message if the build is successful.
# The script is intended to be run from the root of the project directory.
# The script is intended to be run on a Unix-like system with a bash shell, GNU Make, and CMake.
# For other systems, manual execution of the substituted commands is required.
# See LICENSE file for license information.

# * Constants *

# Info log strings
readonly CLEAN_STR="\x1b[0m[\x1b[34m*\x1b[0m] \x1b[34mCleaning build directory...\x1b[0m\n"
readonly BUILD_DIR_STR="\x1b[0m[\x1b[34m*\x1b[0m] \x1b[34mCreating build directory...\x1b[0m\n"
readonly CMAKE_STR="\x1b[0m[\x1b[34m*\x1b[0m] \x1b[34mRunning CMake...\x1b[0m\n"
readonly BUILD_STR="\x1b[0m[\x1b[34m*\x1b[0m] \x1b[34mBuilding...\x1b[0m\n"
readonly COPY_STR="\x1b[0m[\x1b[34m*\x1b[0m] \x1b[34mHardlinking header to build directory...\x1b[0m\n"
readonly DEL_ART_STR="\x1b[0m[\x1b[34m*\x1b[0m] \x1b[34mDeleting build artifacts...\x1b[0m\n"

# Query log string
readonly ASK_INSTALL="\x1b[0m\x1b[33mInstall the library?\x1b[0m [\x1b[32my\x1b[0m/\x1b[31mN\x1b[0m]: "

# Success log string
readonly BUILD_COMPLETE="\x1b[0m[\x1b[32;1m+\x1b[0m] \x1b[32;1mBuild complete!\x1b[0m\n"

# Error log strings
readonly CLEAN_FAIL="\x1b[0m[\x1b[37;41;1m%%\x1b[0m] \x1b[37;41;1mFailed to clean build directory!\x1b[0m\n"
readonly BUILD_DIR_FAIL="\x1b[0m[\x1b[37;41;1m%%\x1b[0m] \x1b[37;41;1mFailed to create build directory!\x1b[0m\n"
readonly CMAKE_FAIL="\x1b[0m[\x1b[37;41;1m%%\x1b[0m] \x1b[37;41;1mFailed to run CMake!\x1b[0m\n"
readonly BUILD_FAIL="\x1b[0m[\x1b[37;41;1m%%\x1b[0m] \x1b[37;41;1mFailed to build using the default build tool (usually GNU Make)!\x1b[0m\n"
readonly COPY_FAIL="\x1b[0m[\x1b[37;41;1m%%\x1b[0m] \x1b[37;41;1mFailed to hardlink header to build directory!\x1b[0m\n"
readonly INSTALL_FAIL="\x1b[0m[\x1b[37;41;1m%%\x1b[0m] \x1b[37;41;1mFailed to install the library!\x1b[0m\n"
readonly DEL_ART_FAIL="\x1b[0m[\x1b[37;41;1m%%\x1b[0m] \x1b[37;41;1mFailed to delete build artifacts!\x1b[0m\n"

# * Procedure *

# Clean build directory if anything is present within it
printf "$CLEAN_STR"
rm -rf ./build/* || 
    { printf "$CLEAN_FAIL" && exit 1; }
# Create build directory if it does not exista and change working directory to it
printf "$BUILD_DIR_STR"
{ mkdir -p build && cd ./build; } || 
    { printf "$BUILD_DIR_FAIL" && exit 1; }
# Run cmake to generate build files
printf "$CMAKE_STR"
cmake .. || 
    { printf "$CMAKE_FAIL" && exit 1; }
# Run make to build the project
printf "$BUILD_STR"
cmake --build . --config Release ||
    { printf "$BUILD_FAIL" && exit 1; }
# Hardlink the header file to the build directory
printf "$COPY_STR"
ln ../include/forestry.h . || 
    { printf "$COPY_FAIL" && exit 1; }
# Ask user if they want to install the library to their system
printf "$ASK_INSTALL"
read -r QUERY_INSTALL
case $QUERY_INSTALL in
    y|Y) sudo cmake --install . || 
        { printf "$INSTALL_FAIL" && exit 1; } ;;
    *) ;;
esac
# Delete build artifacts
printf "$DEL_ART_STR"
rm -rf ./CMakeFiles ./CMakeCache.txt ./Makefile ./cmake_install.cmake || 
    { printf "$DEL_ART_FAIL" && exit 1; }
# Print build success message
printf "$BUILD_COMPLETE"
exit 0
