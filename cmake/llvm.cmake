Set(FETCHCONTENT_QUIET FALSE)
FetchContent_Declare(
        llvm
        URL https://github.com/llvm/llvm-project/releases/download/llvmorg-14.0.6/llvm-14.0.6.src.tar.xz
        SOURCE_SUBDIR  llvm-14.0.6.src
)

set(LLVM_ENABLE_PROJECTS    "lld;libc")

set(LLVM_INCLUDE_TESTS      OFF)
set(LLVM_INCLUDE_EXAMPLES   OFF)
set(LLVM_INCLUDE_BENCHMARKS OFF)
set(LLVM_INCLUDE_DOCS       OFF)
set(LLVM_ENABLE_OCAMLDOC    OFF)

FetchContent_MakeAvailable(llvm)