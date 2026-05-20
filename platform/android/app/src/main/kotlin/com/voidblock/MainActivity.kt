package com.voidblock

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Switch
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.lifecycle.viewmodel.compose.viewModel

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            val statsViewModel: StatsViewModel = viewModel()
            var enabled by remember { mutableStateOf(false) }
            MaterialTheme {
                Scaffold(topBar = { TopAppBar(title = { Text("VoidBlock") }) }) {
                    Column {
                        Switch(
                            checked = enabled,
                            onCheckedChange = { checked ->
                                enabled = checked
                                if (checked) {
                                    startService(android.content.Intent(this@MainActivity, VoidBlockVpnService::class.java))
                                } else {
                                    stopService(android.content.Intent(this@MainActivity, VoidBlockVpnService::class.java))
                                }
                            },
                        )
                        Text("Blocked: ${statsViewModel.blockedCount.collectAsState().value}")
                    }
                }
            }
        }
    }
}
