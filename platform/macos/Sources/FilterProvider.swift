import NetworkExtension
import os.log

final class FilterProvider: NEFilterDataProvider {
    override func handleNewFlow(_ flow: NEFilterFlow) -> NEFilterNewFlowVerdict {
        if let socketFlow = flow as? NEFilterSocketFlow,
           let endpoint = socketFlow.remoteEndpoint as? NWHostEndpoint {
            let host = endpoint.hostname.lowercased()
            if host.contains("doubleclick") || host.contains("adservice") {
                return .drop()
            }
        }
        return .allow()
    }
}
