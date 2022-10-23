Set(FETCHCONTENT_QUIET FALSE)

if(UNIX)
        FetchContent_Declare(
                llvm
                URL https://github.com/llvm/llvm-project/releases/download/llvmorg-14.0.0/clang+llvm-14.0.0-x86_64-linux-gnu-ubuntu-18.04.tar.xz
        )
elseif(APPLE)
        FetchContent_Declare(
                llvm
                URL https://github.com/llvm/llvm-project/releases/download/llvmorg-14.0.0/clang+llvm-14.0.0-x86_64-apple-darwin.tar.xz
        )
else()
        message(FATAL_ERROR "Can not recognise platform, cannot download llvm. \n You can install it manually from source https://github.com/llvm/llvm-project")
endif()

# Check if population has already been performed
FetchContent_GetProperties(llvm)
if(NOT llvm_POPULATED)
        FetchContent_Populate(llvm)

        file(COPY ${llvm_SOURCE_DIR} DESTINATION ${CMAKE_BINARY_DIR})
endif()
