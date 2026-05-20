package com.voidblock

import android.content.Intent
import android.net.VpnService
import android.os.ParcelFileDescriptor
import android.util.Log
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.cancel
import kotlinx.coroutines.launch

class VoidBlockVpnService : VpnService() {
    private var vpnInterface: ParcelFileDescriptor? = null
    private val scope = CoroutineScope(SupervisorJob() + Dispatchers.IO)

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        scope.launch {
            try {
                startVpn()
            } catch (t: Throwable) {
                Log.e("VoidBlockVpnService", "VPN failed", t)
                stopSelf()
            }
        }
        return START_STICKY
    }

    private fun startVpn() {
        val builder = Builder()
        builder.addAddress("10.0.0.2", 32)
        builder.addAddress("fd00:0:0:0:0:0:0:2", 128)
        builder.addRoute("0.0.0.0", 0)
        builder.addRoute("::", 0)
        builder.setSession("VoidBlockTunnel")
        vpnInterface?.close()
        vpnInterface = builder.establish()
    }

    override fun onDestroy() {
        scope.cancel()
        vpnInterface?.close()
        vpnInterface = null
        super.onDestroy()
    }

    override fun onRevoke() {
        scope.cancel()
        vpnInterface?.close()
        vpnInterface = null
        super.onRevoke()
    }
}
