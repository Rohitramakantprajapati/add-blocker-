package com.voidblock

import android.app.Application
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

data class StatsState(
    val blockedCount: Long = 0,
    val batteryPercent: Int = 100,
    val ramMb: Int = 12,
)

class StatsViewModel(application: Application) : AndroidViewModel(application) {
    private val _state = MutableStateFlow(StatsState())
    val state: StateFlow<StatsState> = _state.asStateFlow()

    fun refresh() {
        viewModelScope.launch {
            _state.update {
                it.copy(
                    blockedCount = it.blockedCount + 1,
                    batteryPercent = 97,
                    ramMb = 13,
                )
            }
        }
    }
}
