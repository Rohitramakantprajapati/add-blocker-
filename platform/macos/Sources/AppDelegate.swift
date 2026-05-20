import AppKit
import os.log

@main
final class AppDelegate: NSObject, NSApplicationDelegate {
    private var statusItem: NSStatusItem?

    func applicationDidFinishLaunching(_ notification: Notification) {
        let item = NSStatusBar.system.statusItem(withLength: NSStatusItem.variableLength)
        item.button?.title = "VoidBlock"
        item.menu = buildMenu()
        statusItem = item
    }

    private func buildMenu() -> NSMenu {
        let menu = NSMenu()
        let openItem = menu.addItem(withTitle: "Open VoidBlock", action: #selector(openApp), keyEquivalent: "o")
        openItem.target = self
        menu.addItem(NSMenuItem.separator())
        let quitItem = menu.addItem(withTitle: "Quit", action: #selector(terminateApp), keyEquivalent: "q")
        quitItem.target = self
        return menu
    }

    @objc private func openApp() {
        NSApp.activate(ignoringOtherApps: true)
        os_log("VoidBlock menu opened")
    }

    @objc private func terminateApp() {
        NSApp.terminate(nil)
    }
}
