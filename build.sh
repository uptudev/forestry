#!/bin/sh
readonly CLEAN_STR=
    "\x1b[0m[\x1b[34m*\x1b[0m] \x1b[34mCleaning build directory...\x1b[0m\n"
readonly CLEAN_FAIL=
    "\x1b[0m[\x1b[37;41m%%\x1b[0m] \x1b[37;41;1mFailed to clean build directory!\x1b[0m"
readonly BUILD_DIR_STR=
    "\x1b[0m[\x1b[34m*\x1b[0m] \x1b[34mCreating build directory...\x1b[0m\n"
readonly BUILD_FAIL=
    "\x1b[0m[\x1b[37;41m%%\x1b[0m] \x1b[37;41;1mFailed to create build directory!\x1b[0m"
readonly BUILD_CD_FAIL=
    "\x1b[0m[\x1b[37;41m%%\x1b[0m] \x1b[37;41;1mFailed to cd into build directory!\x1b[0m"
readonly CMAKE_STR=
    "\x1b[0m[\x1b[34m*\x1b[0m] \x1b[34mRunning cmake...\x1b[0m\n"
readonly CMAKE_FAIL=
    "\x1b[0m[\x1b[37;41m%%\x1b[0m] \x1b[37;41;1mFailed to run cmake!\x1b[0m"
readonly MAKE_STR=
    "\x1b[0m[\x1b[34m*\x1b[0m] \x1b[34mRunning make...\x1b[0m\n"
readonly MAKE_FAIL=
    "\x1b[0m[\x1b[37;41m%%\x1b[0m] \x1b[37;41;1mFailed to run make!\x1b[0m"
readonly COPY_STR=
    "\x1b[0m[\x1b[34m*\x1b[0m] \x1b[34mCopying header to build directory...\x1b[0m\n"
readonly COPY_FAIL=
    "\x1b[0m[\x1b[37;41m%%\x1b[0m] \x1b[37;41;1mFailed to copy header to build directory!\x1b[0m"
readonly BUILD_COMPLETE=
    "\x1b[0m[\x1b[32m+\x1b[0m] \x1b[32;1mBuild complete!\x1b[0m"

printf '..%s..' "$CLEAN_STR"
rm -rf ./build || 
    (printf '..%s..' "$CLEAN_FAIL" && exit)
printf '..%s..' "$BUILD_DIR_STR"
mkdir -p build || 
    (printf '..%s..' "$BUILD_FAIL" && exit)
cd build || 
    (printf '..%s..' "$BUILD_CD_FAIL" && exit)
printf '..%s..' "$CMAKE_STR"
cmake .. -DCMAKE_BUILD_TYPE=Release --fresh || 
    (printf '..%s..' "$CMAKE_FAIL" && exit)
printf '..%s..' "$MAKE_STR"
make ||
    (printf '..%s..' "$MAKE_FAIL" && exit)
printf '..%s..' "$COPY_STR"
cp ../src/forestry.h . || 
    (printf '..%s..' "$COPY_FAIL" && exit)
printf '..%s..' "$BUILD_COMPLETE"
