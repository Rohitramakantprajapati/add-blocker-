package com.voidblock

import androidx.lifecycle.ViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow

class StatsViewModel : ViewModel() {
    private val _blockedCount = MutableStateFlow(0L)
    val blockedCount: StateFlow<Long> = _blockedCount

    fun recordBlocked() {
        _blockedCount.value = _blockedCount.value + 1
    }
}
