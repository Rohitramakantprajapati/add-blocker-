package com.voidblock

import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateListOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue

class SplitTunnel {
    private val excludedApps = mutableStateListOf<String>()
    var query by mutableStateOf("")

    val apps: List<String>
        get() = excludedApps

    fun addApp(packageName: String) {
        val trimmed = packageName.trim()
        if (trimmed.isNotEmpty() && !excludedApps.contains(trimmed)) {
            excludedApps.add(trimmed)
        }
    }

    fun removeApp(packageName: String) {
        excludedApps.remove(packageName)
    }
}
