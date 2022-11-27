Set(FETCHCONTENT_QUIET FALSE)

FetchContent_Declare(
        googletest
        GIT_REPOSITORY https://github.com/google/googletest.git
        GIT_TAG        release-1.12.1
        GIT_PROGRESS   FALSE
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
    add_executable(
        ${test_name}
        ${ARGN}
    )
    target_link_libraries(${test_name}
            ${LIB_NAME}
            gtest
            $<TARGET_OBJECTS:test_main>
    )
    add_test(
        NAME ${test_name}
        COMMAND $<TARGET_FILE:${test_name}>
    )
endfunction()
