# cmake -B build -G "Ninja Multi-Config"
# cmake --build build
# # For release (benchmarks)
# cmake --build build --config Release

cmake_minimum_required(VERSION 3.10..4.0)

project(libcrux-ml-dsa
    VERSION 0.1.0
    LANGUAGES C CXX
)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 20)

if(NOT MSVC)
    add_compile_options(
        -Wall
        -fstack-usage
        -Wunused-function
        # -Wno-unused-function
        -Wno-unused-variable
        -fdiagnostics-color=always
        -pedantic
        $<$<CONFIG:DEBUG>:-g>
        $<$<CONFIG:DEBUG>:-Og>
        $<$<CONFIG:RELEASE>:-g>
        $<$<CONFIG:RELEASE>:-O3>
    )
endif(NOT MSVC)

if((CMAKE_C_COMPILER_ID STREQUAL "Clang" AND
    CMAKE_C_COMPILER_VERSION VERSION_GREATER_EQUAL "13.0.0") OR
    (CMAKE_C_COMPILER_ID STREQUAL "AppleClang" AND
    CMAKE_C_COMPILER_VERSION VERSION_GREATER_EQUAL "13.1.6"))
    # add_compile_options(-Werror -Wframe-larger-than=25344)
endif()

set(CMAKE_COLOR_DIAGNOSTICS "ON")
set(CMAKE_EXPORT_COMPILE_COMMANDS 1)
include_directories(
  ${PROJECT_SOURCE_DIR}
  ${PROJECT_SOURCE_DIR}/internal
  ${PROJECT_SOURCE_DIR}/karamel
)
file(GLOB SOURCES
  ${PROJECT_SOURCE_DIR}/combined_core.c
  ${PROJECT_SOURCE_DIR}/libcrux_ct_ops.c
  ${PROJECT_SOURCE_DIR}/libcrux_mldsa44_portable.c
  ${PROJECT_SOURCE_DIR}/libcrux_mldsa65_portable.c
  ${PROJECT_SOURCE_DIR}/libcrux_mldsa87_portable.c
  ${PROJECT_SOURCE_DIR}/libcrux_mldsa_core.c
  ${PROJECT_SOURCE_DIR}/libcrux_mldsa_portable.c
  ${PROJECT_SOURCE_DIR}/libcrux_mlkem512_portable.c
  ${PROJECT_SOURCE_DIR}/libcrux_mlkem768_portable.c
  ${PROJECT_SOURCE_DIR}/libcrux_mlkem1024_portable.c
  ${PROJECT_SOURCE_DIR}/libcrux_mlkem_core.c
  ${PROJECT_SOURCE_DIR}/libcrux_mlkem_portable.c
  ${PROJECT_SOURCE_DIR}/libcrux_sha3_portable.c
)
file(GLOB SOURCES_vec256
  ${PROJECT_SOURCE_DIR}/libcrux_mldsa44_avx2.c
  ${PROJECT_SOURCE_DIR}/libcrux_mldsa65_avx2.c
  ${PROJECT_SOURCE_DIR}/libcrux_mldsa87_avx2.c
  ${PROJECT_SOURCE_DIR}/libcrux_mldsa_avx2.c
  ${PROJECT_SOURCE_DIR}/libcrux_mlkem_avx2.c
  ${PROJECT_SOURCE_DIR}/libcrux_mlkem512_avx2.c
  ${PROJECT_SOURCE_DIR}/libcrux_mlkem768_avx2.c
  ${PROJECT_SOURCE_DIR}/libcrux_mlkem1024_avx2.c
  ${PROJECT_SOURCE_DIR}/libcrux_sha3_avx2.c
)

add_library(combined SHARED ${SOURCES})
add_library(combined_static STATIC ${SOURCES})

if(CMAKE_SYSTEM_PROCESSOR MATCHES "x86_64|amd64|AMD64")
  message(STATUS "Detected an x64 architecture")
  add_compile_definitions(LIBCRUX_X64)

  add_library(combined_vec256 OBJECT ${SOURCES_vec256})
  set_target_properties(combined_vec256 PROPERTIES POSITION_INDEPENDENT_CODE ON)
  target_sources(combined_static PRIVATE $<TARGET_OBJECTS:combined_vec256>)
  target_sources(combined PRIVATE $<TARGET_OBJECTS:combined_vec256>)
endif()

if(CMAKE_SYSTEM_PROCESSOR MATCHES "aarch64|arm64|arm64v8" AND DEFINED ENV{LIBCRUX_NEON})
    message(STATUS "Detected an arm64 architecture")
    add_compile_definitions(LIBCRUX_AARCH64)
endif()


# --- Tests

# Get gtests
include(FetchContent)
FetchContent_Declare(googletest
    GIT_REPOSITORY https://github.com/google/googletest.git
    GIT_TAG v1.16.0
)

# For Windows: Prevent overriding the parent project's compiler/linker settings
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)

# Get nlohmann json
FetchContent_Declare(json
    GIT_REPOSITORY https://github.com/nlohmann/json.git
    GIT_TAG v3.11.3
)
FetchContent_MakeAvailable(json)

add_executable(ml_dsa_test44
    ${PROJECT_SOURCE_DIR}/tests/mldsa44.cc
)
target_link_libraries(ml_dsa_test44 PRIVATE
    combined_static
    gtest_main
    nlohmann_json::nlohmann_json
)

add_executable(ml_dsa_test65
    ${PROJECT_SOURCE_DIR}/tests/mldsa65.cc
)
target_link_libraries(ml_dsa_test65 PRIVATE
    combined_static
    gtest_main
    nlohmann_json::nlohmann_json
)

add_executable(ml_dsa_test87
    ${PROJECT_SOURCE_DIR}/tests/mldsa87.cc
)
target_link_libraries(ml_dsa_test87 PRIVATE
    combined_static
    gtest_main
    nlohmann_json::nlohmann_json
)

add_executable(ml_kem_test512
    ${PROJECT_SOURCE_DIR}/tests/mlkem512.cc
)
target_link_libraries(ml_kem_test512 PRIVATE
    combined_static
    gtest_main
    nlohmann_json::nlohmann_json
)

add_executable(ml_kem_test768
    ${PROJECT_SOURCE_DIR}/tests/mlkem768.cc
)
target_link_libraries(ml_kem_test768 PRIVATE
    combined_static
    gtest_main
    nlohmann_json::nlohmann_json
)

add_executable(ml_kem_test1024
    ${PROJECT_SOURCE_DIR}/tests/mlkem1024.cc
)
target_link_libraries(ml_kem_test1024 PRIVATE
    combined_static
    gtest_main
    nlohmann_json::nlohmann_json
)

if(DEFINED ENV{LIBCRUX_KYBER})
    add_executable(kyber_test
        ${PROJECT_SOURCE_DIR}/tests/kyber768.cc
    )
    target_link_libraries(kyber_test PRIVATE
        gtest_main
        nlohmann_json::nlohmann_json
    )
endif(DEFINED ENV{LIBCRUX_KYBER})

add_executable(sha3_test
    ${PROJECT_SOURCE_DIR}/tests/sha3.cc
)
target_link_libraries(sha3_test PRIVATE
    combined_static
    gtest_main
    nlohmann_json::nlohmann_json
)

# add_executable(kyber_test
#     ${PROJECT_SOURCE_DIR}/tests/kyber768.cc
# )
# target_link_libraries(kyber_test PRIVATE
#     gtest_main
#     nlohmann_json::nlohmann_json
# )

# add_executable(sha3_test
#     ${PROJECT_SOURCE_DIR}/tests/sha3.cc
# )
# target_link_libraries(sha3_test PRIVATE
#     gtest_main
#     nlohmann_json::nlohmann_json
# )

# # --- Benchmarks
# if(DEFINED ENV{LIBCRUX_BENCHMARKS})
#     FetchContent_Declare(benchmark
#         GIT_REPOSITORY https://github.com/google/benchmark.git
#         GIT_TAG v1.9.2
#     )
#     FetchContent_MakeAvailable(benchmark)

#     add_executable(ml_dsa_bench
#         ${PROJECT_SOURCE_DIR}/benches/mldsa768.cc
#     )
#     target_link_libraries(ml_dsa_bench PRIVATE
#         benchmark::benchmark
#     )

#     if(DEFINED ENV{SYMCRYPT_PATH})
#         message("Symcrypt path: $ENV{SYMCRYPT_PATH}")
#         add_compile_definitions(LIBCRUX_SYMCRYPT)
#         target_include_directories(ml_dsa_bench PRIVATE $ENV{SYMCRYPT_PATH})
#         target_link_directories(ml_dsa_bench PRIVATE $ENV{SYMCRYPT_PATH}/bin/lib)
#         target_link_libraries(ml_dsa_bench PRIVATE symcrypt)
#     endif(DEFINED ENV{SYMCRYPT_PATH})

#     add_executable(ml_dsa_keygen
#         ${PROJECT_SOURCE_DIR}/benches/mldsa768_keygen.cc
#     )
#     target_link_libraries(ml_dsa_keygen PRIVATE
#         benchmark::benchmark
#     )

#     add_executable(ml_dsa_encaps
#         ${PROJECT_SOURCE_DIR}/benches/mldsa768_encaps.cc
#     )
#     target_link_libraries(ml_dsa_encaps PRIVATE
#         benchmark::benchmark
#     )

#     if(NOT MSVC)
#         # We benchmark internal functions here that are inlined and thus not available
#         # in MSVC.
#         add_executable(sha3_bench
#             ${PROJECT_SOURCE_DIR}/benches/sha3.cc
#         )
#         target_link_libraries(sha3_bench PRIVATE
#             benchmark::benchmark
#         )
#     endif(NOT MSVC)
# endif(DEFINED ENV{LIBCRUX_BENCHMARKS})
