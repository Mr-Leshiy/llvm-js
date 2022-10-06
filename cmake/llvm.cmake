Set(FETCHCONTENT_QUIET FALSE)
FetchContent_Declare(
        llvm
        GIT_REPOSITORY https://github.com/llvm/llvm-project.git
        GIT_TAG        release/14.x
        GIT_PROGRESS   FALSE
        SOURCE_SUBDIR  llvm
)

set(LLVM_ENABLE_PROJECTS    "lld;libc")

set(LLVM_INCLUDE_TESTS      OFF)
set(LLVM_INCLUDE_EXAMPLES   OFF)
set(LLVM_INCLUDE_BENCHMARKS OFF)
set(LLVM_INCLUDE_DOCS       OFF)
set(LLVM_ENABLE_OCAMLDOC    OFF)

FetchContent_MakeAvailable(llvm)