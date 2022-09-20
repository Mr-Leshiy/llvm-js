cmake_minimum_required(VERSION 3.5)

set(LIB_NAME llvm-js-lib)
set(VERSION 0.1)

project(${LIB_NAME} VERSION ${VERSION} LANGUAGES C CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

option(TESTING      "Build tests" ON)

include(FetchContent)

add_subdirectory(c)
include_directories(c)

if(TESTING)
    include(cmake/gtest.cmake)
    enable_testing()
    add_subdirectory(c/test)
endif()