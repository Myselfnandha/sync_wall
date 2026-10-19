package com.syncwall.app.data

import android.content.Context
import android.os.Build
import com.syncwall.app.wallpaper.WallpaperSetter
import com.syncwall.app.wallpaper.WallpaperTarget
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import uniffi.sync_wall_core.PairingResult
import uniffi.sync_wall_core.SyncReport
import uniffi.sync_wall_core.SyncWallEngineBinding
import uniffi.sync_wall_core.WallpaperMetadata
import java.io.File

object EngineManager {

    private val scope = CoroutineScope(SupervisorJob() + Dispatchers.Default)
    private var engine: SyncWallEngineBinding? = null

    private val _wallpapers = MutableStateFlow<List<WallpaperMetadata>>(emptyList())
    val wallpapers: StateFlow<List<WallpaperMetadata>> = _wallpapers.asStateFlow()

    private val _activeWallpaperPath = MutableStateFlow<String?>(null)
    val activeWallpaperPath: StateFlow<String?> = _activeWallpaperPath.asStateFlow()

    private val _isSyncing = MutableStateFlow(false)
    val isSyncing: StateFlow<Boolean> = _isSyncing.asStateFlow()

    private val _deviceId = MutableStateFlow("")
    val deviceId: StateFlow<String> = _deviceId.asStateFlow()

    private val _lastSyncStatus = MutableStateFlow("Never synced")
    val lastSyncStatus: StateFlow<String> = _lastSyncStatus.asStateFlow()

    private var serverPeerUrl: String? = null

    fun init(context: Context) {
        if (engine != null) return

        try {
            val baseDir = File(context.filesDir, "sync_wall").apply { mkdirs() }
            val deviceModel = "${Build.MANUFACTURER} ${Build.MODEL}"
            val binding = SyncWallEngineBinding(baseDir.absolutePath, deviceModel)
            engine = binding
            _deviceId.value = binding.getDeviceId()

            // Load saved server URL if any
            val prefs = context.getSharedPreferences("sync_wall_prefs", Context.MODE_PRIVATE)
            serverPeerUrl = prefs.getString("server_peer_url", null)

            refreshWallpapers()
            refreshActiveWallpaper()
        } catch (e: Exception) {
            e.printStackTrace()
        }
    }

    fun refreshWallpapers() {
        val eng = engine ?: return
        scope.launch {
            try {
                val list = eng.listWallpapers()
                _wallpapers.value = list
            } catch (e: Exception) {
                e.printStackTrace()
            }
        }
    }

    fun refreshActiveWallpaper() {
        val eng = engine ?: return
        scope.launch {
            val path = eng.getActiveWallpaperPath("primary")
            _activeWallpaperPath.value = path
        }
    }

    suspend fun pair(
        context: Context,
        serverUrl: String,
        token: String,
        width: Int,
        height: Int
    ): Result<PairingResult> = withContext(Dispatchers.IO) {
        runCatching {
            val eng = engine ?: throw IllegalStateException("Engine not initialized")
            val deviceModel = "${Build.MANUFACTURER} ${Build.MODEL}"
            val result = eng.pairWithServer(
                serverUrl = serverUrl,
                token = token,
                deviceName = deviceModel,
                screenW = width.toUInt(),
                screenH = height.toUInt()
            )
            if (result.success) {
                serverPeerUrl = serverUrl
                context.getSharedPreferences("sync_wall_prefs", Context.MODE_PRIVATE)
                    .edit()
                    .putString("server_peer_url", serverUrl)
                    .putString("server_name", result.serverName)
                    .apply()
                // Immediately pull changes
                sync(context)
            }
            result
        }
    }

    suspend fun sync(context: Context): Result<SyncReport> = withContext(Dispatchers.IO) {
        runCatching {
            val eng = engine ?: throw IllegalStateException("Engine not initialized")
            val url = serverPeerUrl ?: throw IllegalStateException("No paired server configured")

            _isSyncing.value = true
            try {
                val report = eng.syncWithPeer(url)
                _lastSyncStatus.value = "Synced ${report.appliedImages} images, ${report.appliedAssignments} assignments"
                refreshWallpapers()
                refreshActiveWallpaper()
                report
            } finally {
                _isSyncing.value = false
            }
        }
    }

    suspend fun setWallpaper(
        context: Context,
        hash: String,
        target: WallpaperTarget = WallpaperTarget.BOTH
    ): Result<Unit> = withContext(Dispatchers.IO) {
        runCatching {
            val eng = engine ?: throw IllegalStateException("Engine not initialized")
            val metrics = context.resources.displayMetrics
            val w = metrics.widthPixels.toUInt()
            val h = metrics.heightPixels.toUInt()

            // Update assignment in engine
            eng.setWallpaper("primary", hash, "fill")

            // Render derivative tailored for phone screen
            val renderedPath = eng.renderWallpaper("primary", w, h, "webp")

            // Apply via Android WallpaperManager
            WallpaperSetter.applyWallpaper(context, renderedPath, target).getOrThrow()

            _activeWallpaperPath.value = renderedPath
        }
    }

    suspend fun fetchBingDaily(context: Context): Result<WallpaperMetadata> = withContext(Dispatchers.IO) {
        runCatching {
            val eng = engine ?: throw IllegalStateException("Engine not initialized")
            val meta = eng.fetchBingDaily()
            refreshWallpapers()
            meta
        }
    }
}
