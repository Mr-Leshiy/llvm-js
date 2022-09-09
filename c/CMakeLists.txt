include_directories(${CMAKE_CURRENT_LIST_DIR})

set(SOURCES
        variable.c
        )

add_library(${LIB_NAME} STATIC ${SOURCES})
