import Foundation
import os.log

struct DNSFallback {
    func setSystemDoH(enabled: Bool) async throws {
        let task = Process()
        task.executableURL = URL(fileURLWithPath: "/usr/sbin/networksetup")
        task.arguments = enabled ? ["-setdnsservers", "Wi-Fi", "1.1.1.1", "9.9.9.9"] : ["-setdnsservers", "Wi-Fi", "Empty"]

        let pipe = Pipe()
        task.standardError = pipe
        task.standardOutput = pipe
        try task.run()
        task.waitUntilExit()

        if task.terminationStatus != 0 {
            let data = pipe.fileHandleForReading.readDataToEndOfFile()
            let message = String(data: data, encoding: .utf8) ?? "unknown error"
            os_log("VoidBlock DNS fallback failed: %{public}@", message)
            throw NSError(domain: "VoidBlock.DNSFallback", code: Int(task.terminationStatus), userInfo: [NSLocalizedDescriptionKey: message])
        }
    }
}
