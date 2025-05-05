cmake_minimum_required(VERSION 3.15)

project(vcmp-python-plugin)

include_directories(pybind11/include)
add_subdirectory(pybind11)

if(WIN32)
    message(STATUS "Detected Windows platform")
    add_definitions(-D_WIN32_WINNT=0x0601)
    set(CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS ON)
    set(PLATFORM_LIBS ws2_32)
else()
    message(STATUS "Detected non-Windows platform")
    set(PLATFORM_LIBS pthread)
endif()

add_library(${PROJECT_NAME} SHARED ./src/main.cpp)
target_link_libraries(${PROJECT_NAME} PRIVATE pybind11::embed ${PLATFORM_LIBS})

math(EXPR _CMAKE_BITS "${CMAKE_SIZEOF_VOID_P} * 8")
set(PYTHON_VER "default")
execute_process(
    COMMAND "python" -c "import sys; print(''.join(str(v) for v in sys.version_info[:2]))"
    OUTPUT_VARIABLE PYTHON_VER
    OUTPUT_STRIP_TRAILING_WHITESPACE
)
string(REGEX REPLACE "\n" "" PYTHON_VER ${PYTHON_VER})
message(STATUS "Python version: ${PYTHON_VER}")

set_target_properties(${PROJECT_NAME} PROPERTIES
    OUTPUT_NAME_DEBUG "${PROJECT_NAME}-cpy${PYTHON_VER}-dbg${_CMAKE_BITS}"
    OUTPUT_NAME_RELEASE "${PROJECT_NAME}-cpy${PYTHON_VER}-rel${_CMAKE_BITS}"
    PREFIX ""
)