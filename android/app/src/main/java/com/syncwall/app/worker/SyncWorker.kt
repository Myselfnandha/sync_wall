package com.syncwall.app.worker

import android.content.Context
import androidx.work.CoroutineWorker
import androidx.work.WorkerParameters
import com.syncwall.app.data.EngineManager
import com.syncwall.app.wallpaper.WallpaperTarget
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

class SyncWorker(
    appContext: Context,
    params: WorkerParameters
) : CoroutineWorker(appContext, params) {

    override suspend fun doWork(): Result = withContext(Dispatchers.IO) {
        try {
            EngineManager.init(applicationContext)

            // Step 1: Sync with desktop peer
            val syncResult = EngineManager.sync(applicationContext)

            // Step 2: Check if auto-rotation is enabled
            val prefs = applicationContext.getSharedPreferences("sync_wall_prefs", Context.MODE_PRIVATE)
            val autoRotate = prefs.getBoolean("auto_rotate_enabled", false)

            if (autoRotate) {
                val wallpapers = EngineManager.wallpapers.value
                if (wallpapers.isNotEmpty()) {
                    val randomWallpaper = wallpapers.random()
                    EngineManager.setWallpaper(applicationContext, randomWallpaper.hash, WallpaperTarget.BOTH)
                }
            }

            if (syncResult.isSuccess) {
                Result.success()
            } else {
                Result.retry()
            }
        } catch (e: Exception) {
            e.printStackTrace()
            Result.retry()
        }
    }
}
