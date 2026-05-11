package com.voidblock

class DnsProxy {
    external fun nativeResolve(query: ByteArray): ByteArray

    fun resolveSafely(query: ByteArray): ByteArray {
        return try {
            nativeResolve(query)
        } catch (throwable: Throwable) {
            query
        }
    }
}
