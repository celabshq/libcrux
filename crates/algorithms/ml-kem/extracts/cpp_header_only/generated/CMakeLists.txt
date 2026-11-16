# cmake -B build -G "Ninja Multi-Config"
# cmake --build build
# # For release (benchmarks)
# cmake --build build --config Release

cmake_minimum_required(VERSION 3.10..4.0)

project(libcrux-ml-kem
    VERSION 0.1.0
    LANGUAGES C CXX
)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

include(mlkem.cmake)

include_directories(
    ${PROJECT_SOURCE_DIR}
    ${PROJECT_SOURCE_DIR}/internal
    ${PROJECT_SOURCE_DIR}/intrinsics
    ${PROJECT_SOURCE_DIR}/karamel/include
)

if((CMAKE_C_COMPILER_ID STREQUAL "Clang" AND
    CMAKE_C_COMPILER_VERSION VERSION_GREATER_EQUAL "13.0.0") OR
    (CMAKE_C_COMPILER_ID STREQUAL "AppleClang" AND
    CMAKE_C_COMPILER_VERSION VERSION_GREATER_EQUAL "13.1.6"))
    add_compile_options(-Werror -Wframe-larger-than=25344)
endif()
