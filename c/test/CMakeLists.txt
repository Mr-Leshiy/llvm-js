include_directories(${CMAKE_CURRENT_LIST_DIR})
include_directories(${CMAKE_BINARY_DIR}/c)

add_library(test_main OBJECT test_main.cpp)
target_link_libraries(test_main gtest)

addtest(garbage_collector_test garbage_collector_test.cpp)
addtest(variable_test variable_test.cpp)
addtest(object_test object_test.cpp)
addtest(number_test number_test.cpp)
addtest(array_test array_test.cpp)

addtest(fmt_test fmt_test.cpp)
