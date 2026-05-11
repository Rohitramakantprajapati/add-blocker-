package com.voidblock

import android.net.VpnService
import android.os.ParcelFileDescriptor
import android.util.Log
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.cancel
import kotlinx.coroutines.delay
import kotlinx.coroutines.isActive
import kotlinx.coroutines.launch

class VpnServiceImpl : VpnService() {
    private val serviceJob = SupervisorJob()
    private val serviceScope = CoroutineScope(Dispatchers.IO + serviceJob)
    private var tunInterface: ParcelFileDescriptor? = null

    override fun onCreate() {
        super.onCreate()
        serviceScope.launch {
            startTunnel()
        }
    }

    override fun onDestroy() {
        try {
            tunInterface?.close()
        } catch (throwable: Throwable) {
            Log.e("VoidBlock", "Failed to close TUN", throwable)
        } finally {
            tunInterface = null
            serviceScope.cancel()
            serviceJob.cancel()
            super.onDestroy()
        }
    }

    private suspend fun startTunnel() {
        while (serviceJob.isActive) {
            try {
                if (tunInterface == null) {
                    tunInterface = Builder()
                        .setSession("VoidBlock")
                        .addAddress("10.0.0.2", 24)
                        .addAddress("fd00:void:1::2", 64)
                        .addRoute("0.0.0.0", 0)
                        .addRoute("::", 0)
                        .setBlocking(true)
                        .establish()
                }
                delay(1000)
            } catch (throwable: Throwable) {
                Log.e("VoidBlock", "VPN loop failed", throwable)
                delay(1000)
            }
        }
    }
}
