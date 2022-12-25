cmake_minimum_required(VERSION 3.19)

project(SimpleLib)

include_directories(include)

file(GLOB_RECURSE sources "src/*.c" "src/*.cpp")

add_library(${PROJECT_NAME} STATIC ${sources})