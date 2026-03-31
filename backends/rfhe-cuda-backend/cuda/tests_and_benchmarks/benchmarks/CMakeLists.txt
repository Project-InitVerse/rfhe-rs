project(benchmark_rfhe_cuda_backend LANGUAGES CXX)

# See if the minimum CUDA version is available. If not, only enable documentation building.
set(MINIMUM_SUPPORTED_CUDA_VERSION 10.0)
include(CheckLanguage)
# See if CUDA is available
check_language(CUDA)
# If so, enable CUDA to check the version.
if(CMAKE_CUDA_COMPILER)
  enable_language(CUDA)
  find_package(CUDAToolkit)
endif()
# If CUDA is not available, or the minimum version is too low do not build
if(NOT CMAKE_CUDA_COMPILER)
  message(FATAL_ERROR "Cuda compiler not found.")
endif()

if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif()

# Disable the Google Benchmark requirement on Google Test
set(BENCHMARK_ENABLE_GTEST_TESTS OFF)
set(BENCHMARK_ENABLE_TESTING OFF)

include(FetchContent)
FetchContent_Declare(
  googlebenchmark
  GIT_REPOSITORY https://github.com/google/benchmark.git
  GIT_TAG v1.7.1)
FetchContent_MakeAvailable(googlebenchmark)

# Enable ExternalProject CMake module
include(ExternalProject)

set(CONCRETE_CUDA_SOURCE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../../")

# Enable ExternalProject CMake module
include(ExternalProject)
set(RFHE_RS_SOURCE_DIR "${CMAKE_BINARY_DIR}/../../../../")
set(RFHE_RS_BINARY_DIR "${RFHE_RS_SOURCE_DIR}/target/release")

if(NOT TARGET rfhe-rs)
  ExternalProject_Add(
    rfhe-rs
    SOURCE_DIR ${RFHE_RS_SOURCE_DIR}
    BUILD_IN_SOURCE 1
    BUILD_ALWAYS 1
    UPDATE_COMMAND ""
    CONFIGURE_COMMAND ""
    DOWNLOAD_COMMAND ""
    BUILD_COMMAND $(MAKE) build_c_api
    INSTALL_COMMAND ""
    LOG_BUILD ON)
endif()

include_directories(${CMAKE_CURRENT_SOURCE_DIR}/../include)
include_directories(${CONCRETE_CUDA_SOURCE_DIR}/include)
include_directories(${CONCRETE_CUDA_SOURCE_DIR}/src)
include_directories(${RFHE_RS_BINARY_DIR})
include_directories(${RFHE_RS_BINARY_DIR}/deps)
include_directories("${CMAKE_CURRENT_SOURCE_DIR}")

find_package(OpenMP REQUIRED)
# Add the OpenMP flag to the compiler flags
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${OpenMP_CXX_FLAGS}")

add_library(rfhe_rs_lib STATIC IMPORTED)
add_dependencies(rfhe_rs_lib rfhe-rs)
set_target_properties(rfhe_rs_lib PROPERTIES IMPORTED_LOCATION ${RFHE_RS_BINARY_DIR}/librfhe.a)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wl,--no-as-needed -ldl")

set(BINARY benchmark_rfhe_cuda_backend)

file(
  GLOB_RECURSE BENCH_SOURCES
  LIST_DIRECTORIES false
  benchmark*.cpp main.cpp)

add_executable(${BINARY} ${BENCH_SOURCES} ../utils.cpp ../setup_and_teardown.cpp)

set_target_properties(benchmark_rfhe_cuda_backend PROPERTIES CUDA_SEPARABLE_COMPILATION ON CUDA_RESOLVE_DEVICE_SYMBOLS
                                                                                           ON)
target_link_libraries(
  benchmark_rfhe_cuda_backend
  PUBLIC benchmark::benchmark rfhe_rs_lib rfhe_cuda_backend OpenMP::OpenMP_CXX
  PRIVATE CUDA::cudart)
