#include <windows.h>

#include <string>

namespace {

constexpr const wchar_t* kRegistryPath = L"SYSTEM\\CurrentControlSet\\Services\\Dnscache\\Parameters";
constexpr const wchar_t* kValueName = L"ServerAddresses";

bool write_registry_string(const std::wstring& value) {
    HKEY key = nullptr;
    if (RegCreateKeyExW(HKEY_LOCAL_MACHINE, kRegistryPath, 0, nullptr, 0, KEY_WRITE, nullptr, &key, nullptr) != ERROR_SUCCESS) {
        return false;
    }

    const LONG status = RegSetValueExW(
        key,
        kValueName,
        0,
        REG_SZ,
        reinterpret_cast<const BYTE*>(value.c_str()),
        static_cast<DWORD>((value.size() + 1) * sizeof(wchar_t)));

    RegCloseKey(key);
    return status == ERROR_SUCCESS;
}

} // namespace

extern "C" bool voidblock_set_dns_override() {
    return write_registry_string(L"127.0.0.1");
}

extern "C" bool voidblock_restore_dns_override() {
    return write_registry_string(L"");
}
