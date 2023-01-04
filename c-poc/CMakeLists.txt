cmake_minimum_required(VERSION 3.19)

project(SimpleApp)

file(REAL_PATH "../simple-lib" simpleLibPath)
add_subdirectory(${simpleLibPath} build)

add_executable(${PROJECT_NAME} main.cpp)

include_directories("../simple-lib/include")
target_link_libraries(${PROJECT_NAME} SimpleLib)
