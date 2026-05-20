package com.voidblock

import android.util.Log

object DnsProxy {
    external fun routeDns(packet: ByteArray): Int

    fun safeRoute(packet: ByteArray): Boolean {
        return try {
            try {
                System.loadLibrary("voidblock_jni")
            } catch (throwable: Throwable) {
                Log.e("DnsProxy", "JNI library load failed", throwable)
                return false
            }
            routeDns(packet) == 0
        } catch (throwable: Throwable) {
            Log.e("DnsProxy", "JNI routeDns failed", throwable)
            false
        }
    }
}
