Set(FETCHCONTENT_QUIET FALSE)

message(STATUS "System name: ${CMAKE_SYSTEM_NAME}")
message(STATUS "System platform: ${CMAKE_HOST_SYSTEM_PROCESSOR}")

if(CMAKE_SYSTEM_NAME EQUAL "Ubuntu" AND CMAKE_HOST_SYSTEM_PROCESSOR EQUAL "x86_64")
        FetchContent_Declare(
                llvm
                URL https://github.com/llvm/llvm-project/releases/download/llvmorg-14.0.0/clang+llvm-14.0.0-x86_64-linux-gnu-ubuntu-18.04.tar.xz
        )
        set(LLVM_DEFINED TRUE)
endif()


if(CMAKE_SYSTEM_NAME EQUAL "Darwin" AND CMAKE_HOST_SYSTEM_PROCESSOR EQUAL "x86_64")
        FetchContent_Declare(
                llvm
                URL https://github.com/llvm/llvm-project/releases/download/llvmorg-14.0.0/clang+llvm-14.0.0-x86_64-apple-darwin.tar.xz
        )
        set(LLVM_DEFINED TRUE)
endif()

if(LLVM_DEFINED EQUAL TRUE)
        # Check if population has already been performed
        FetchContent_GetProperties(llvm)
        if(NOT llvm_POPULATED)
                FetchContent_Populate(llvm)
        
                file(COPY ${llvm_SOURCE_DIR} DESTINATION ${CMAKE_BINARY_DIR})
        endif()
else()
        message(STATUS "Install llvm from source")
        FetchContent_Declare(
                llvm
                URL https://github.com/llvm/llvm-project/releases/download/llvmorg-14.0.0/llvm-project-14.0.0.src.tar.xz
        )

        set(LLVM_INCLUDE_TESTS      OFF)
        set(LLVM_INCLUDE_EXAMPLES   OFF)
        set(LLVM_INCLUDE_BENCHMARKS OFF)
        set(LLVM_INCLUDE_DOCS       OFF)
        set(LLVM_ENABLE_OCAMLDOC    OFF)


        # Check if population has already been performed
        FetchContent_GetProperties(llvm)
        if(NOT llvm_POPULATED)
                FetchContent_Populate(llvm)

                # Bring the populated content into the build
                add_subdirectory(${llvm_SOURCE_DIR}/llvm ${CMAKE_BINARY_DIR}/llvm-build)
        endif()
endif()
