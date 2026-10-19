package com.syncwall.app.ui.screens

import android.widget.Toast
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.journeyapps.barcodescanner.ScanContract
import com.journeyapps.barcodescanner.ScanOptions
import com.syncwall.app.data.EngineManager
import com.syncwall.app.ui.theme.*
import kotlinx.coroutines.launch
import org.json.JSONObject

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun PairScreen() {
    val context = LocalContext.current
    val scope = rememberCoroutineScope()
    val deviceId by EngineManager.deviceId.collectAsState()

    var hostInput by remember { mutableStateOf("192.168.1.") }
    var portInput by remember { mutableStateOf("7443") }
    var tokenInput by remember { mutableStateOf("") }
    var isPairing by remember { mutableStateOf(false) }

    val barcodeLauncher = rememberLauncherForActivityResult(ScanContract()) { result ->
        if (result.contents != null) {
            val qrText = result.contents
            try {
                // Try parsing JSON payload from desktop QR
                val json = JSONObject(qrText)
                val host = json.optString("host", "")
                val port = json.optInt("port", 7443)
                val token = json.optString("token", "")

                if (host.isNotEmpty() && token.isNotEmpty()) {
                    hostInput = host
                    portInput = port.toString()
                    tokenInput = token

                    // Automatically trigger pairing
                    scope.launch {
                        isPairing = true
                        val metrics = context.resources.displayMetrics
                        val serverUrl = "https://$host:$port"
                        val pairRes = EngineManager.pair(context, serverUrl, token, metrics.widthPixels, metrics.heightPixels)
                        isPairing = false

                        if (pairRes.isSuccess) {
                            Toast.makeText(context, "Paired successfully with ${pairRes.getOrNull()?.serverName}!", Toast.LENGTH_LONG).show()
                        } else {
                            Toast.makeText(context, "Pairing failed: ${pairRes.exceptionOrNull()?.message}", Toast.LENGTH_LONG).show()
                        }
                    }
                } else {
                    tokenInput = qrText
                }
            } catch (e: Exception) {
                tokenInput = qrText
            }
        }
    }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .background(Slate900)
            .verticalScroll(rememberScrollState())
            .padding(16.dp)
    ) {
        Text(
            text = "Pair with Desktop",
            style = MaterialTheme.typography.headlineLarge,
            color = Slate50
        )
        Text(
            text = "Run 'sync_wall pair' on your Linux desktop to show the QR code",
            style = MaterialTheme.typography.bodyMedium,
            color = Slate400
        )

        Spacer(modifier = Modifier.height(20.dp))

        // QR Code Scanner Action Card
        Card(
            modifier = Modifier.fillMaxWidth(),
            shape = RoundedCornerShape(16.dp),
            colors = CardDefaults.cardColors(containerColor = Slate800)
        ) {
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(20.dp),
                horizontalAlignment = Alignment.CenterHorizontally
            ) {
                Icon(
                    imageVector = Icons.Default.QrCodeScanner,
                    contentDescription = null,
                    tint = Emerald400,
                    modifier = Modifier.size(56.dp)
                )
                Spacer(modifier = Modifier.height(12.dp))
                Text(
                    text = "Scan Desktop QR Code",
                    style = MaterialTheme.typography.titleLarge,
                    color = Slate50
                )
                Text(
                    text = "Instantly configure host, port, and pairing credentials",
                    style = MaterialTheme.typography.bodyMedium,
                    color = Slate400,
                    modifier = Modifier.padding(horizontal = 16.dp, vertical = 6.dp)
                )
                Spacer(modifier = Modifier.height(16.dp))

                FilledTonalButton(
                    onClick = {
                        val options = ScanOptions().apply {
                            setDesiredBarcodeFormats(ScanOptions.QR_CODE)
                            setPrompt("Scan sync_wall pairing QR on your Linux desktop")
                            setBeepEnabled(false)
                            setOrientationLocked(true)
                        }
                        barcodeLauncher.launch(options)
                    },
                    modifier = Modifier
                        .fillMaxWidth()
                        .height(48.dp),
                    colors = ButtonDefaults.filledTonalButtonColors(containerColor = Emerald600, contentColor = Slate50)
                ) {
                    Icon(imageVector = Icons.Default.CameraAlt, contentDescription = null, modifier = Modifier.size(20.dp))
                    Spacer(modifier = Modifier.width(8.dp))
                    Text("Start QR Scanner")
                }
            }
        }

        Spacer(modifier = Modifier.height(24.dp))

        Text(
            text = "OR ENTER MANUALLY",
            style = MaterialTheme.typography.labelSmall,
            fontWeight = FontWeight.Bold,
            color = Slate400
        )

        Spacer(modifier = Modifier.height(12.dp))

        // Manual Input Card
        Card(
            modifier = Modifier.fillMaxWidth(),
            shape = RoundedCornerShape(16.dp),
            colors = CardDefaults.cardColors(containerColor = Slate800)
        ) {
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(16.dp),
                verticalArrangement = Arrangement.spacedBy(12.dp)
            ) {
                OutlinedTextField(
                    value = hostInput,
                    onValueChange = { hostInput = it },
                    label = { Text("Desktop IP Address") },
                    placeholder = { Text("e.g. 192.168.1.100") },
                    singleLine = true,
                    modifier = Modifier.fillMaxWidth(),
                    colors = OutlinedTextFieldDefaults.colors(
                        focusedBorderColor = Emerald400,
                        unfocusedBorderColor = Slate600,
                        focusedTextColor = Slate50,
                        unfocusedTextColor = Slate200
                    )
                )

                OutlinedTextField(
                    value = portInput,
                    onValueChange = { portInput = it },
                    label = { Text("Port") },
                    placeholder = { Text("7443") },
                    singleLine = true,
                    modifier = Modifier.fillMaxWidth(),
                    colors = OutlinedTextFieldDefaults.colors(
                        focusedBorderColor = Emerald400,
                        unfocusedBorderColor = Slate600,
                        focusedTextColor = Slate50,
                        unfocusedTextColor = Slate200
                    )
                )

                OutlinedTextField(
                    value = tokenInput,
                    onValueChange = { tokenInput = it },
                    label = { Text("128-bit Token or 6-digit PIN") },
                    placeholder = { Text("Paste token or enter PIN") },
                    singleLine = true,
                    modifier = Modifier.fillMaxWidth(),
                    colors = OutlinedTextFieldDefaults.colors(
                        focusedBorderColor = Emerald400,
                        unfocusedBorderColor = Slate600,
                        focusedTextColor = Slate50,
                        unfocusedTextColor = Slate200
                    )
                )

                Button(
                    onClick = {
                        scope.launch {
                            isPairing = true
                            val metrics = context.resources.displayMetrics
                            val serverUrl = "https://${hostInput.trim()}:${portInput.trim()}"
                            val pairRes = EngineManager.pair(
                                context,
                                serverUrl,
                                tokenInput.trim(),
                                metrics.widthPixels,
                                metrics.heightPixels
                            )
                            isPairing = false

                            if (pairRes.isSuccess) {
                                Toast.makeText(context, "Paired successfully with ${pairRes.getOrNull()?.serverName}!", Toast.LENGTH_LONG).show()
                            } else {
                                Toast.makeText(context, "Pairing failed: ${pairRes.exceptionOrNull()?.message}", Toast.LENGTH_LONG).show()
                            }
                        }
                    },
                    enabled = !isPairing && hostInput.isNotBlank() && tokenInput.isNotBlank(),
                    modifier = Modifier
                        .fillMaxWidth()
                        .height(48.dp),
                    colors = ButtonDefaults.buttonColors(containerColor = Emerald600, contentColor = Slate50)
                ) {
                    if (isPairing) {
                        CircularProgressIndicator(modifier = Modifier.size(20.dp), color = Slate50, strokeWidth = 2.dp)
                        Spacer(modifier = Modifier.width(8.dp))
                        Text("Pairing with Desktop...")
                    } else {
                        Text("Connect and Pair")
                    }
                }
            }
        }

        Spacer(modifier = Modifier.height(20.dp))

        // Device Identity Fingerprint
        Card(
            modifier = Modifier.fillMaxWidth(),
            shape = RoundedCornerShape(12.dp),
            colors = CardDefaults.cardColors(containerColor = Slate950)
        ) {
            Column(modifier = Modifier.padding(12.dp)) {
                Text(
                    text = "This Device ID (mTLS Fingerprint):",
                    style = MaterialTheme.typography.labelSmall,
                    color = Slate400
                )
                Spacer(modifier = Modifier.height(4.dp))
                Text(
                    text = if (deviceId.isNotEmpty()) deviceId else "Generating...",
                    style = MaterialTheme.typography.bodyMedium,
                    fontFamily = FontFamily.Monospace,
                    color = Cyan400,
                    fontSize = 11.sp
                )
            }
        }
    }
}
