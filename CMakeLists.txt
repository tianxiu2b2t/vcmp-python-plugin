cmake_minimum_required(VERSION 3.15)

# 项目名称
project(vcmp-python-plugin)

# 包含 pybind11
include_directories(pybind11/include)
add_subdirectory(pybind11)

# 检测平台
if(WIN32)
    message(STATUS "Detected Windows platform")
    add_definitions(-D_WIN32_WINNT=0x0601)
    set(CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS ON)
    set(PLATFORM_LIBS ws2_32)
else()
    message(STATUS "Detected non-Windows platform")
    set(PLATFORM_LIBS pthread)
endif()

# 添加共享库
add_library(${PROJECT_NAME} SHARED ./src/main.cpp)
target_link_libraries(${PROJECT_NAME} PRIVATE pybind11::embed ${PLATFORM_LIBS})

# 检测 Python 版本
find_package(Python3 REQUIRED COMPONENTS Interpreter)
set(PYTHON_EXECUTABLE ${Python3_EXECUTABLE})
execute_process(
    COMMAND ${PYTHON_EXECUTABLE} -c "import sys; print(''.join(str(v) for v in sys.version_info[:2]))"
    OUTPUT_VARIABLE PYTHON_VER
    OUTPUT_STRIP_TRAILING_WHITESPACE
)
string(REGEX REPLACE "\n" "" PYTHON_VER ${PYTHON_VER})
message(STATUS "Python version: ${PYTHON_VER}")

# 设置目标属性
math(EXPR _CMAKE_BITS "${CMAKE_SIZEOF_VOID_P} * 8")
set_target_properties(${PROJECT_NAME} PROPERTIES
    OUTPUT_NAME_DEBUG "${PROJECT_NAME}-cpy${PYTHON_VER}-dbg${_CMAKE_BITS}"
    OUTPUT_NAME_RELEASE "${PROJECT_NAME}-cpy${PYTHON_VER}-rel${_CMAKE_BITS}"
    PREFIX ""
)

# RPATH 配置
if(NOT WIN32)
    set_target_properties(${PROJECT_NAME} PROPERTIES
        INSTALL_RPATH_USE_LINK_PATH TRUE
        INSTALL_RPATH "$ORIGIN:/usr/lib:/usr/local/lib"
    )
endif()

# 指定编译器（如有需要）
if(CMAKE_CXX_COMPILER_VERSION LESS 12)
    message(WARNING "Your compiler version is lower than recommended. Consider upgrading GCC or Clang.")
endif()
