include_directories(${CMAKE_CURRENT_LIST_DIR})

set(SOURCES
        variable/variable.cpp
        number/number.cpp
        object/object.cpp
        array/array.cpp
        function/function.cpp
        interface.cpp
        )

add_library(${LIB_NAME} STATIC ${SOURCES})
