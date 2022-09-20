include_directories(${CMAKE_CURRENT_LIST_DIR})

set(SOURCES
        variable.cpp
        assertions.cpp
        logical.cpp
        )

add_library(${LIB_NAME} STATIC ${SOURCES})
