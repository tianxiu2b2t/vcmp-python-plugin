cmake_minimum_required(VERSION 4.0)

project(vcmp-python-plugin)

include_directories(pybind11/include)

add_subdirectory(pybind11)

add_library(${PROJECT_NAME} SHARED ./src/main.cpp)
target_link_libraries(${PROJECT_NAME} PRIVATE pybind11::embed)

math(EXPR _CMAKE_BITS "${CMAKE_SIZEOF_VOID_P} * 8")

#execute_process(COMMAND "${PYTHON_EXECUTABLE}" "-c" "import sys;print(''.join(str(v) for v in sys.version_info[:2]));"
#    OUTPUT_VARIABLE PYTHON_VER)
#string(REGEX REPLACE "\n" "" PYTHON_VER ${PYTHON_VER})

set_target_properties(${PROJECT_NAME} PROPERTIES OUTPUT_NAME_DEBUG "${PROJECT_NAME}-cpy${PYTHON_VER}-dbg${_CMAKE_BITS}")
set_target_properties(${PROJECT_NAME} PROPERTIES OUTPUT_NAME_RELEASE "${PROJECT_NAME}-cpy${PYTHON_VER}-rel${_CMAKE_BITS}")
set_target_properties(${PROJECT_NAME} PROPERTIES PREFIX "")
