cmake_minimum_required(VERSION 3.16)

project(nodio VERSION 0.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

set(CMAKE_INCLUDE_CURRENT_DIR ON)

add_subdirectory(rtaudio)

add_executable(nodio main.cpp)

target_include_directories(nodio PUBLIC rtaudio)
target_link_libraries(nodio rtaudio)
