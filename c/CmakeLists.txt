include_directories(${CMAKE_CURRENT_LIST_DIR})

set(SOURCES
        variable.c
        )

set(BUILD STATIC)
add_library(${LIB_NAME} ${BUILD} ${SOURCES})