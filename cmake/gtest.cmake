FetchContent_Declare(
        googletest
        GIT_REPOSITORY https://github.com/google/googletest.git
        GIT_TAG        release-1.12.1
        GIT_SHALLOW    TRUE
        GIT_PROGRESS   TRUE
)

 # Prevent overriding the parent project's compiler/linker
# settings on Windows
set(gtest_force_shared_crt ON)
set(BUILD_GMOCK OFF)
set(BUILD_GTEST ON)
set(INSTALL_GTEST OFF)

FetchContent_MakeAvailable(googletest)

if(CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
    # using gcc
    target_compile_options(gtest PRIVATE
        -Wno-maybe-uninitialized
    )
endif()

function(addtest test_name)
    set(THREADS_PREFER_PTHREAD_FLAG TRUE)
    find_package(Threads REQUIRED)
    
    add_executable(${test_name} ${ARGN})
    addtest_part(${test_name} ${ARGN})
    target_link_libraries(${test_name}
            gtest_main
            gtest
            Threads::Threads
            )
    add_test(
            NAME ${test_name}
            COMMAND $<TARGET_FILE:${test_name}>
    )
    set_target_properties(${test_name} PROPERTIES
            CXX_STANDARD 17
            CXX_STANDARD_REQUIRED TRUE
            )
    disable_clang_tidy(${test_name})
    if(UNIX)
        # works only on UNIX systems
        target_compile_options(${test_name} PUBLIC
                # we don't care about potential null dereferences in tests
                -Wno-null-dereference
                )
    endif()
endfunction()

function(addtest_part test_name)
    if (POLICY CMP0076)
        cmake_policy(SET CMP0076 NEW)
    endif ()
    target_sources(${test_name} PUBLIC
            ${ARGN}
            )
endfunction()

function(disable_clang_tidy target)
    set_target_properties(${target} PROPERTIES
            C_CLANG_TIDY ""
            CXX_CLANG_TIDY ""
            )
endfunction()