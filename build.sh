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

readonly CLEAN_STR="\x1b[0m[\x1b[34m*\x1b[0m] \x1b[34mCleaning build directory...\x1b[0m\n"
readonly CLEAN_FAIL="\x1b[0m[\x1b[37;41m%%\x1b[0m] \x1b[37;41;1mFailed to clean build directory!\x1b[0m\n"
readonly BUILD_DIR_STR="\x1b[0m[\x1b[34m*\x1b[0m] \x1b[34mCreating build directory...\x1b[0m\n"
readonly BUILD_FAIL="\x1b[0m[\x1b[37;41m%%\x1b[0m] \x1b[37;41;1mFailed to create build directory!\x1b[0m\n"
readonly BUILD_CD_FAIL="\x1b[0m[\x1b[37;41m%%\x1b[0m] \x1b[37;41;1mFailed to cd into build directory!\x1b[0m\n"
readonly CMAKE_STR="\x1b[0m[\x1b[34m*\x1b[0m] \x1b[34mRunning cmake...\x1b[0m\n"
readonly CMAKE_FAIL="\x1b[0m[\x1b[37;41m%%\x1b[0m] \x1b[37;41;1mFailed to run cmake!\x1b[0m\n"
readonly MAKE_STR="\x1b[0m[\x1b[34m*\x1b[0m] \x1b[34mRunning make...\x1b[0m\n"
readonly MAKE_FAIL="\x1b[0m[\x1b[37;41m%%\x1b[0m] \x1b[37;41;1mFailed to run make!\x1b[0m\n"
readonly COPY_STR="\x1b[0m[\x1b[34m*\x1b[0m] \x1b[34mHardlinking header to build directory...\x1b[0m\n"
readonly COPY_FAIL="\x1b[0m[\x1b[37;41m%%\x1b[0m] \x1b[37;41;1mFailed to hardlink header to build directory!\x1b[0m\n"
readonly ASK_INSTALL="\x1b[0m\x1b[33mInstall the library?\x1b[0m [\x1b[32my\x1b[0m/\x1b[31mN\x1b[0m]: "
readonly INSTALL_FAIL="\x1b[0m[\x1b[37;41m%%\x1b[0m] \x1b[37;41;1mFailed to install the library!\x1b[0m\n"
readonly DEL_ART_STR="\x1b[0m[\x1b[34m*\x1b[0m] \x1b[34mDeleting build artifacts...\x1b[0m\n"
readonly DEL_ART_FAIL="\x1b[0m[\x1b[37;41m%%\x1b[0m] \x1b[37;41;1mFailed to delete build artifacts!\x1b[0m\n"
readonly BUILD_COMPLETE="\x1b[0m[\x1b[32m+\x1b[0m] \x1b[32;1mBuild complete!\x1b[0m\n"

# Clean build directory if it exists
printf "$CLEAN_STR"
rm -rf ./build || 
    (printf "$CLEAN_FAIL" && exit)
# Create build directory
printf "$BUILD_DIR_STR"
mkdir -p build || 
    (printf "$BUILD_FAIL" && exit)
# Change working directory to build directory
cd build || 
    (printf "$BUILD_CD_FAIL" && exit)
# Run cmake to generate build files
printf "$CMAKE_STR"
cmake .. -DCMAKE_BUILD_TYPE=Release --fresh || 
    (printf "$CMAKE_FAIL" && exit)
# Run make to build the project
printf "$MAKE_STR"
make ||
    (printf "$MAKE_FAIL" && exit)
# Hardlink the header file to the build directory
printf "$COPY_STR"
ln ../include/forestry.h . || 
    (printf "$COPY_FAIL" && exit)
# Ask user if they want to install the library to their system
printf "$ASK_INSTALL"
read -r QUERY_INSTALL
case $QUERY_INSTALL in
    y|Y) sudo make install || 
        (printf "$INSTALL_FAIL" && exit) ;;
    *) ;;
esac
# Delete build artifacts
printf "$DEL_ART_STR"
rm -rf ./CMakeFiles ./CMakeCache.txt ./Makefile ./cmake_install.cmake || 
    (printf "$DEL_ART_FAIL" && exit)
# Print build success message
printf "$BUILD_COMPLETE"
