include_directories(${CMAKE_CURRENT_LIST_DIR})

set(SOURCES
        variable/variable.cpp
        number/number.cpp
        object/object.cpp
        array/array.cpp
        assertions.cpp
        # logical.cpp
        interface.cpp
        )

add_library(${LIB_NAME} STATIC ${SOURCES})
