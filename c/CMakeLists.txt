include_directories(${CMAKE_CURRENT_LIST_DIR})

set(SOURCES
        variable.cpp
        object.cpp
        array.cpp
        assertions.cpp
        logical.cpp
        arithmetic.cpp
        )

add_library(${LIB_NAME} STATIC ${SOURCES})
