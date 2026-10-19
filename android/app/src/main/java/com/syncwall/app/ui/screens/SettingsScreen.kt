package com.syncwall.app.ui.screens

import android.content.Context
import android.widget.Toast
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
import androidx.compose.ui.unit.dp
import com.syncwall.app.data.EngineManager
import com.syncwall.app.ui.theme.*
import java.io.File

@Composable
fun SettingsScreen() {
    val context = LocalContext.current
    val prefs = remember { context.getSharedPreferences("sync_wall_prefs", Context.MODE_PRIVATE) }

    var autoRotate by remember { mutableStateOf(prefs.getBoolean("auto_rotate_enabled", false)) }
    var rotateIntervalHours by remember { mutableStateOf(prefs.getInt("rotate_interval_hours", 6)) }
    val lastSyncStatus by EngineManager.lastSyncStatus.collectAsState()

    // Storage calculation
    val baseDir = remember { File(context.filesDir, "sync_wall") }
    var storageText by remember { mutableStateOf("Calculating...") }

    fun calculateStorage() {
        val totalBytes = baseDir.walkTopDown().filter { it.isFile }.map { it.length() }.sum()
        val mb = totalBytes.toDouble() / (1024 * 1024)
        storageText = String.format("%.2f MB", mb)
    }

    LaunchedEffect(Unit) {
        calculateStorage()
    }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .background(Slate900)
            .verticalScroll(rememberScrollState())
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        Text(
            text = "Settings",
            style = MaterialTheme.typography.headlineLarge,
            color = Slate50
        )

        // Wallpaper Rotation Card
        Card(
            modifier = Modifier.fillMaxWidth(),
            shape = RoundedCornerShape(16.dp),
            colors = CardDefaults.cardColors(containerColor = Slate800)
        ) {
            Column(modifier = Modifier.padding(16.dp)) {
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Column(modifier = Modifier.weight(1f)) {
                        Text(
                            text = "Auto-Rotate Wallpaper",
                            style = MaterialTheme.typography.titleLarge,
                            color = Slate50
                        )
                        Text(
                            text = "Randomly changes wallpaper from synced collection",
                            style = MaterialTheme.typography.bodyMedium,
                            color = Slate400
                        )
                    }
                    Switch(
                        checked = autoRotate,
                        onCheckedChange = {
                            autoRotate = it
                            prefs.edit().putBoolean("auto_rotate_enabled", it).apply()
                        },
                        colors = SwitchDefaults.colors(
                            checkedThumbColor = Slate50,
                            checkedTrackColor = Emerald500
                        )
                    )
                }

                if (autoRotate) {
                    Spacer(modifier = Modifier.height(16.dp))
                    Text(
                        text = "Rotation Interval: Every $rotateIntervalHours hours",
                        style = MaterialTheme.typography.bodyMedium,
                        color = Slate200
                    )
                    Spacer(modifier = Modifier.height(8.dp))
                    Row(
                        modifier = Modifier.fillMaxWidth(),
                        horizontalArrangement = Arrangement.spacedBy(8.dp)
                    ) {
                        listOf(1, 3, 6, 12, 24).forEach { hours ->
                            FilterChip(
                                selected = rotateIntervalHours == hours,
                                onClick = {
                                    rotateIntervalHours = hours
                                    prefs.edit().putInt("rotate_interval_hours", hours).apply()
                                },
                                label = { Text("${hours}h") },
                                colors = FilterChipDefaults.filterChipColors(
                                    selectedContainerColor = Emerald600,
                                    selectedLabelColor = Slate50,
                                    containerColor = Slate700,
                                    labelColor = Slate200
                                )
                            )
                        }
                    }
                }
            }
        }

        // Synchronization Status Card
        Card(
            modifier = Modifier.fillMaxWidth(),
            shape = RoundedCornerShape(16.dp),
            colors = CardDefaults.cardColors(containerColor = Slate800)
        ) {
            Column(modifier = Modifier.padding(16.dp)) {
                Text(
                    text = "Background Synchronization",
                    style = MaterialTheme.typography.titleLarge,
                    color = Slate50
                )
                Spacer(modifier = Modifier.height(4.dp))
                Text(
                    text = "Syncs with paired desktop in the background via WorkManager",
                    style = MaterialTheme.typography.bodyMedium,
                    color = Slate400
                )
                Spacer(modifier = Modifier.height(12.dp))
                Row(verticalAlignment = Alignment.CenterVertically) {
                    Icon(
                        imageVector = Icons.Default.Info,
                        contentDescription = null,
                        tint = Cyan400,
                        modifier = Modifier.size(18.dp)
                    )
                    Spacer(modifier = Modifier.width(8.dp))
                    Text(
                        text = lastSyncStatus,
                        style = MaterialTheme.typography.bodyMedium,
                        color = Cyan400
                    )
                }
            }
        }

        // Storage & Cache Card
        Card(
            modifier = Modifier.fillMaxWidth(),
            shape = RoundedCornerShape(16.dp),
            colors = CardDefaults.cardColors(containerColor = Slate800)
        ) {
            Column(modifier = Modifier.padding(16.dp)) {
                Text(
                    text = "Local Storage & Cache",
                    style = MaterialTheme.typography.titleLarge,
                    color = Slate50
                )
                Spacer(modifier = Modifier.height(4.dp))
                Text(
                    text = "Total synced images and rendered derivatives: $storageText",
                    style = MaterialTheme.typography.bodyMedium,
                    color = Slate400
                )
                Spacer(modifier = Modifier.height(16.dp))

                OutlinedButton(
                    onClick = {
                        val derivedDir = File(baseDir, "derived")
                        if (derivedDir.exists()) {
                            derivedDir.deleteRecursively()
                            derivedDir.mkdirs()
                        }
                        calculateStorage()
                        Toast.makeText(context, "Derivative cache cleared", Toast.LENGTH_SHORT).show()
                    },
                    modifier = Modifier.height(48.dp)
                ) {
                    Icon(imageVector = Icons.Default.DeleteOutline, contentDescription = null, modifier = Modifier.size(18.dp))
                    Spacer(modifier = Modifier.width(8.dp))
                    Text("Purge Rendered Cache")
                }
            }
        }

        // Engine & App Info
        Card(
            modifier = Modifier.fillMaxWidth(),
            shape = RoundedCornerShape(16.dp),
            colors = CardDefaults.cardColors(containerColor = Slate800)
        ) {
            Column(modifier = Modifier.padding(16.dp)) {
                Text(
                    text = "sync_wall v0.1.0",
                    style = MaterialTheme.typography.titleLarge,
                    color = Slate50
                )
                Spacer(modifier = Modifier.height(4.dp))
                Text(
                    text = "Core: Rust sync_wall_core (UniFFI, BLAKE3, HLC, mTLS)\nBackend: Android WallpaperManager.setBitmap",
                    style = MaterialTheme.typography.bodyMedium,
                    color = Slate400
                )
            }
        }
    }
}
