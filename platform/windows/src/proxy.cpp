#include <winsock2.h>
#include <ws2tcpip.h>

#include <atomic>
#include <chrono>
#include <cstdint>
#include <csignal>
#include <iostream>
#include <stdexcept>
#include <string>
#include <thread>

#pragma comment(lib, "Ws2_32.lib")

namespace {

class WinsockSession {
public:
    WinsockSession() {
        WSADATA data{};
        if (WSAStartup(MAKEWORD(2, 2), &data) != 0) {
            throw std::runtime_error("WSAStartup failed");
        }
    }

    ~WinsockSession() {
        WSACleanup();
    }
};

std::atomic_bool g_running{true};

void handle_signal(int) {
    g_running.store(false);
}

void run_local_proxy() {
    while (g_running.load()) {
        std::this_thread::sleep_for(std::chrono::milliseconds(250));
    }
}

} // namespace

int main() {
    try {
        std::signal(SIGINT, handle_signal);
        std::signal(SIGTERM, handle_signal);
        WinsockSession session;
        std::cout << "VoidBlock Windows proxy starting\n";
        run_local_proxy();
        std::cout << "VoidBlock Windows proxy stopping\n";
        return 0;
    } catch (const std::exception& error) {
        std::cerr << "VoidBlock proxy error: " << error.what() << '\n';
        return 1;
    }
}
