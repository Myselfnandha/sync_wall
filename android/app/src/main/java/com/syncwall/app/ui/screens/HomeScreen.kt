package com.syncwall.app.ui.screens

import android.widget.Toast
import androidx.compose.animation.AnimatedVisibility
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.foundation.lazy.grid.items
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import coil.compose.AsyncImage
import coil.request.ImageRequest
import com.syncwall.app.data.EngineManager
import com.syncwall.app.ui.theme.*
import com.syncwall.app.wallpaper.WallpaperTarget
import kotlinx.coroutines.launch
import uniffi.sync_wall_core.WallpaperMetadata
import java.io.File

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun HomeScreen() {
    val context = LocalContext.current
    val scope = rememberCoroutineScope()

    val wallpapers by EngineManager.wallpapers.collectAsState()
    val activePath by EngineManager.activeWallpaperPath.collectAsState()
    val isSyncing by EngineManager.isSyncing.collectAsState()

    var selectedWallpaper by remember { mutableStateOf<WallpaperMetadata?>(null) }
    var showTargetDialog by remember { mutableStateOf(false) }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .background(Slate900)
            .padding(16.dp)
    ) {
        // Top Header
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(vertical = 8.dp),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Column {
                Text(
                    text = "sync_wall",
                    style = MaterialTheme.typography.headlineLarge,
                    color = Slate50
                )
                Text(
                    text = "${wallpapers.size} synced wallpapers",
                    style = MaterialTheme.typography.bodyMedium,
                    color = Slate400
                )
            }

            Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                // Fetch Bing Daily
                IconButton(
                    onClick = {
                        scope.launch {
                            Toast.makeText(context, "Fetching Bing daily photo...", Toast.LENGTH_SHORT).show()
                            val res = EngineManager.fetchBingDaily(context)
                            if (res.isSuccess) {
                                Toast.makeText(context, "Added Bing wallpaper!", Toast.LENGTH_SHORT).show()
                            } else {
                                Toast.makeText(context, "Fetch failed: ${res.exceptionOrNull()?.message}", Toast.LENGTH_SHORT).show()
                            }
                        }
                    },
                    modifier = Modifier.size(48.dp)
                ) {
                    Icon(
                        imageVector = Icons.Default.CloudDownload,
                        contentDescription = "Fetch Bing Daily",
                        tint = Cyan400
                    )
                }

                // Sync Now Button
                FilledTonalButton(
                    onClick = {
                        scope.launch {
                            val res = EngineManager.sync(context)
                            if (res.isSuccess) {
                                Toast.makeText(context, "Synced successfully", Toast.LENGTH_SHORT).show()
                            } else {
                                Toast.makeText(context, "Sync: ${res.exceptionOrNull()?.message}", Toast.LENGTH_SHORT).show()
                            }
                        }
                    },
                    enabled = !isSyncing,
                    modifier = Modifier.height(48.dp),
                    colors = ButtonDefaults.filledTonalButtonColors(
                        containerColor = Emerald600,
                        contentColor = Slate50
                    )
                ) {
                    if (isSyncing) {
                        CircularProgressIndicator(
                            modifier = Modifier.size(18.dp),
                            color = Slate50,
                            strokeWidth = 2.dp
                        )
                        Spacer(modifier = Modifier.width(8.dp))
                        Text("Syncing...")
                    } else {
                        Icon(imageVector = Icons.Default.Sync, contentDescription = null, modifier = Modifier.size(18.dp))
                        Spacer(modifier = Modifier.width(6.dp))
                        Text("Sync")
                    }
                }
            }
        }

        Spacer(modifier = Modifier.height(12.dp))

        // Active Wallpaper Hero Card
        if (activePath != null && File(activePath!!).exists()) {
            Card(
                modifier = Modifier
                    .fillMaxWidth()
                    .height(140.dp),
                shape = RoundedCornerShape(16.dp),
                colors = CardDefaults.cardColors(containerColor = Slate800)
            ) {
                Row(
                    modifier = Modifier
                        .fillMaxSize()
                        .padding(12.dp),
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    AsyncImage(
                        model = ImageRequest.Builder(context)
                            .data(File(activePath!!))
                            .crossfade(true)
                            .build(),
                        contentDescription = "Active Wallpaper",
                        contentScale = ContentScale.Crop,
                        modifier = Modifier
                            .width(80.dp)
                            .fillMaxHeight()
                            .clip(RoundedCornerShape(12.dp))
                    )

                    Spacer(modifier = Modifier.width(16.dp))

                    Column(modifier = Modifier.weight(1f)) {
                        Row(verticalAlignment = Alignment.CenterVertically) {
                            Box(
                                modifier = Modifier
                                    .size(8.dp)
                                    .background(Emerald400, CircleShape)
                            )
                            Spacer(modifier = Modifier.width(6.dp))
                            Text(
                                text = "CURRENT WALLPAPER",
                                style = MaterialTheme.typography.labelSmall,
                                color = Emerald400,
                                fontWeight = FontWeight.Bold
                            )
                        }
                        Spacer(modifier = Modifier.height(4.dp))
                        Text(
                            text = File(activePath!!).name,
                            style = MaterialTheme.typography.titleLarge,
                            color = Slate50,
                            maxLines = 1
                        )
                        Text(
                            text = "Cropped natively for your display",
                            style = MaterialTheme.typography.bodyMedium,
                            color = Slate400
                        )
                    }
                }
            }
            Spacer(modifier = Modifier.height(16.dp))
        }

        // Wallpaper Grid
        if (wallpapers.isEmpty()) {
            Box(
                modifier = Modifier
                    .weight(1f)
                    .fillMaxWidth(),
                contentAlignment = Alignment.Center
            ) {
                Column(horizontalAlignment = Alignment.CenterHorizontally) {
                    Icon(
                        imageVector = Icons.Default.Wallpaper,
                        contentDescription = null,
                        tint = Slate600,
                        modifier = Modifier.size(64.dp)
                    )
                    Spacer(modifier = Modifier.height(12.dp))
                    Text(
                        text = "No synced wallpapers yet",
                        style = MaterialTheme.typography.titleLarge,
                        color = Slate400
                    )
                    Spacer(modifier = Modifier.height(6.dp))
                    Text(
                        text = "Pair with your Linux desktop or fetch Bing photo",
                        style = MaterialTheme.typography.bodyMedium,
                        color = Slate600
                    )
                }
            }
        } else {
            LazyVerticalGrid(
                columns = GridCells.Fixed(2),
                modifier = Modifier.weight(1f),
                horizontalArrangement = Arrangement.spacedBy(12.dp),
                verticalArrangement = Arrangement.spacedBy(12.dp)
            ) {
                items(wallpapers, key = { it.hash }) { wallpaper ->
                    WallpaperCard(
                        wallpaper = wallpaper,
                        onClick = {
                            selectedWallpaper = wallpaper
                            showTargetDialog = true
                        }
                    )
                }
            }
        }
    }

    // Target Selection Dialog (Home, Lock, or Both)
    if (showTargetDialog && selectedWallpaper != null) {
        AlertDialog(
            onDismissRequest = { showTargetDialog = false },
            title = { Text(text = "Set as Wallpaper", color = Slate50) },
            text = {
                Column(modifier = Modifier.fillMaxWidth()) {
                    Text(
                        text = selectedWallpaper!!.name,
                        style = MaterialTheme.typography.bodyMedium,
                        color = Slate400
                    )
                    Spacer(modifier = Modifier.height(16.dp))

                    FilledTonalButton(
                        onClick = {
                            scope.launch {
                                showTargetDialog = false
                                Toast.makeText(context, "Applying wallpaper...", Toast.LENGTH_SHORT).show()
                                val res = EngineManager.setWallpaper(context, selectedWallpaper!!.hash, WallpaperTarget.BOTH)
                                if (res.isSuccess) {
                                    Toast.makeText(context, "Wallpaper updated!", Toast.LENGTH_SHORT).show()
                                } else {
                                    Toast.makeText(context, "Error: ${res.exceptionOrNull()?.message}", Toast.LENGTH_LONG).show()
                                }
                            }
                        },
                        modifier = Modifier
                            .fillMaxWidth()
                            .height(48.dp),
                        colors = ButtonDefaults.filledTonalButtonColors(containerColor = Emerald600, contentColor = Slate50)
                    ) {
                        Text("Home & Lock Screen")
                    }

                    Spacer(modifier = Modifier.height(8.dp))

                    OutlinedButton(
                        onClick = {
                            scope.launch {
                                showTargetDialog = false
                                EngineManager.setWallpaper(context, selectedWallpaper!!.hash, WallpaperTarget.SYSTEM)
                            }
                        },
                        modifier = Modifier
                            .fillMaxWidth()
                            .height(48.dp)
                    ) {
                        Text("Home Screen Only", color = Slate200)
                    }

                    Spacer(modifier = Modifier.height(8.dp))

                    OutlinedButton(
                        onClick = {
                            scope.launch {
                                showTargetDialog = false
                                EngineManager.setWallpaper(context, selectedWallpaper!!.hash, WallpaperTarget.LOCK)
                            }
                        },
                        modifier = Modifier
                            .fillMaxWidth()
                            .height(48.dp)
                    ) {
                        Text("Lock Screen Only", color = Slate200)
                    }
                }
            },
            confirmButton = {},
            dismissButton = {
                TextButton(onClick = { showTargetDialog = false }) {
                    Text("Cancel", color = Slate400)
                }
            },
            containerColor = Slate800
        )
    }
}

@Composable
fun WallpaperCard(
    wallpaper: WallpaperMetadata,
    onClick: () -> Unit
) {
    val context = LocalContext.current
    val baseDir = File(context.filesDir, "sync_wall")
    val blobFile = File(baseDir, "objects/${wallpaper.hash.substring(0, 2)}/${wallpaper.hash.substring(2)}")

    Card(
        modifier = Modifier
            .fillMaxWidth()
            .height(240.dp)
            .clickable(onClick = onClick),
        shape = RoundedCornerShape(14.dp),
        colors = CardDefaults.cardColors(containerColor = Slate800)
    ) {
        Box(modifier = Modifier.fillMaxSize()) {
            AsyncImage(
                model = ImageRequest.Builder(context)
                    .data(blobFile)
                    .crossfade(true)
                    .build(),
                contentDescription = wallpaper.name,
                contentScale = ContentScale.Crop,
                modifier = Modifier.fillMaxSize()
            )

            // Gradient overlay at bottom
            Box(
                modifier = Modifier
                    .fillMaxWidth()
                    .align(Alignment.BottomCenter)
                    .background(Slate900.copy(alpha = 0.85f))
                    .padding(8.dp)
            ) {
                Column {
                    Text(
                        text = wallpaper.name,
                        style = MaterialTheme.typography.bodyMedium,
                        fontWeight = FontWeight.SemiBold,
                        color = Slate50,
                        maxLines = 1
                    )
                    Row(
                        modifier = Modifier.fillMaxWidth(),
                        horizontalArrangement = Arrangement.SpaceBetween
                    ) {
                        Text(
                            text = "${wallpaper.width}x${wallpaper.height}",
                            style = MaterialTheme.typography.labelSmall,
                            color = Slate400
                        )
                        Text(
                            text = "Set",
                            style = MaterialTheme.typography.labelSmall,
                            fontWeight = FontWeight.Bold,
                            color = Emerald400
                        )
                    }
                }
            }
        }
    }
}
