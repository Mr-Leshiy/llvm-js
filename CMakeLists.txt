cmake_minimum_required(VERSION 3.5)

set(LIB_NAME llvm-js)
set(VERSION 0.1)

project(${LIB_NAME} VERSION ${VERSION} LANGUAGES C CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

option(TESTING      "Build tests"   OFF)
option(LLVM         "Build llvm"    ON)

include(FetchContent)

add_subdirectory(c)
include_directories(c)

if(LLVM)
    include(cmake/llvm.cmake)
endif()

if(TESTING)
    include(cmake/gtest.cmake)
    enable_testing()
    add_subdirectory(c/test)
endif()