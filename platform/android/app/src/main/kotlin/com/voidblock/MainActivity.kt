package com.voidblock

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.Checkbox
import androidx.compose.material3.Divider
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Surface
import androidx.compose.material3.Switch
import androidx.compose.material3.Text
import androidx.compose.material3.dynamicDarkColorScheme
import androidx.compose.material3.dynamicLightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            val context = LocalContext.current
            val darkTheme = true
            val scheme = if (darkTheme) dynamicDarkColorScheme(context) else dynamicLightColorScheme(context)
            MaterialTheme(colorScheme = scheme) {
                Surface(modifier = Modifier.fillMaxSize()) {
                    Dashboard()
                }
            }
        }
    }
}

@Composable
private fun Dashboard() {
    var blockingEnabled by remember { mutableStateOf(true) }
    var aiEnabled by remember { mutableStateOf(false) }
    val viewModel = remember { StatsViewModel(application = android.app.Application()) }
    val stats by viewModel.state.collectAsState()
    val splitTunnel = remember { SplitTunnel() }

    Column(modifier = Modifier.fillMaxSize().padding(20.dp), verticalArrangement = Arrangement.spacedBy(16.dp)) {
        Text("VoidBlock", style = MaterialTheme.typography.headlineLarge)
        Row(horizontalArrangement = Arrangement.spacedBy(12.dp)) {
            Text("Blocking")
            Switch(checked = blockingEnabled, onCheckedChange = { blockingEnabled = it })
        }
        Row(horizontalArrangement = Arrangement.spacedBy(12.dp)) {
            Text("AI")
            Checkbox(checked = aiEnabled, onCheckedChange = { aiEnabled = it })
        }
        Card(modifier = Modifier.fillMaxWidth()) {
            Column(modifier = Modifier.padding(16.dp), verticalArrangement = Arrangement.spacedBy(8.dp)) {
                Text("Blocked ${stats.blockedCount}")
                Text("Battery ${stats.batteryPercent}%")
                Text("RAM ${stats.ramMb} MB")
                Button(onClick = { viewModel.refresh() }) { Text("Refresh") }
            }
        }
        Divider()
        OutlinedTextField(value = splitTunnel.query, onValueChange = { splitTunnel.query = it }, label = { Text("Exclude app package") }, modifier = Modifier.fillMaxWidth())
        Button(onClick = { splitTunnel.addApp(splitTunnel.query) }) { Text("Add exclusion") }
        Spacer(modifier = Modifier.height(4.dp))
        Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
            splitTunnel.apps.forEach { app -> Text(app) }
        }
    }
}
