import NetworkExtension
import os.log

final class AppProxyProvider: NEAppProxyProvider {
    override func startProxy(options: [String : Any]?, completionHandler: @escaping (Error?) -> Void) {
        os_log("VoidBlock app proxy starting")
        completionHandler(nil)
    }

    override func stopProxy(with reason: NEProviderStopReason, completionHandler: @escaping () -> Void) {
        os_log("VoidBlock app proxy stopping: %{public}@", String(describing: reason))
        completionHandler()
    }

    override func handleNewFlow(_ flow: NEAppProxyFlow) -> Bool {
        completionHandler(.closeReadWrite())
        return true
    }

    private func completionHandler(_ verdict: NEAppProxyFlowVerdict) {
        os_log("VoidBlock verdict: %{public}@", String(describing: verdict))
    }
}
