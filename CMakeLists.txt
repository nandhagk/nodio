cmake_minimum_required(VERSION 3.16)

project(nodio VERSION 0.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

# Configure Qt
find_package(Qt6 REQUIRED COMPONENTS Core Widgets Gui OpenGL)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
# set_property(DIRECTORY APPEND PROPERTY COMPILE_DEFINITIONS $<$<CONFIG:Debug>:QT_QML_DEBUG>)
set(CMAKE_INCLUDE_CURRENT_DIR ON)

add_subdirectory(rtaudio)
add_subdirectory(nodeeditor)

set(CALC_SOURCE_FILES
  main.cpp
  MathOperationDataModel.cpp
  NumberDisplayDataModel.cpp
  NumberSourceDataModel.cpp
)

set(CALC_HEADER_FILES
  AdditionModel.hpp
  DivisionModel.hpp
  DecimalData.hpp
  MathOperationDataModel.hpp
  NumberDisplayDataModel.hpp
  NumberSourceDataModel.hpp
  SubtractionModel.hpp
)

add_executable(nodio
  ${CALC_SOURCE_FILES}
  ${CALC_HEAEDR_FILES}
)

target_include_directories(nodio PUBLIC rtaudio QtNodes)
target_link_libraries(nodio rtaudio QtNodes)
