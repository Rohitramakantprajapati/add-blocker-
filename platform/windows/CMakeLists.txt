cmake_minimum_required(VERSION 3.20)
project(VoidBlock LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(voidblock-proxy
    src/proxy.cpp
    src/dns_override.cpp
)

target_include_directories(voidblock-proxy PRIVATE src)
target_link_libraries(voidblock-proxy PRIVATE ws2_32 advapi32)
