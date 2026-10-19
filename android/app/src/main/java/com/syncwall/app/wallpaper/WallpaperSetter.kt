package com.syncwall.app.wallpaper

import android.app.WallpaperManager
import android.content.Context
import android.graphics.Bitmap
import android.graphics.BitmapFactory
import android.os.Build
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import java.io.File

enum class WallpaperTarget {
    SYSTEM,
    LOCK,
    BOTH
}

object WallpaperSetter {

    suspend fun applyWallpaper(
        context: Context,
        imagePath: String,
        target: WallpaperTarget = WallpaperTarget.BOTH
    ): Result<Unit> = withContext(Dispatchers.IO) {
        runCatching {
            val file = File(imagePath)
            if (!file.exists()) {
                throw IllegalArgumentException("Wallpaper file does not exist at $imagePath")
            }

            // Decode dimensions first to ensure safe memory allocation
            val options = BitmapFactory.Options().apply {
                inJustDecodeBounds = true
            }
            BitmapFactory.decodeFile(imagePath, options)

            // Calculate safe inSampleSize (prevent mobile OOM)
            val displayMetrics = context.resources.displayMetrics
            val reqWidth = displayMetrics.widthPixels
            val reqHeight = displayMetrics.heightPixels

            var sampleSize = 1
            if (options.outHeight > reqHeight || options.outWidth > reqWidth) {
                val halfHeight = options.outHeight / 2
                val halfWidth = options.outWidth / 2
                while ((halfHeight / sampleSize) >= reqHeight && (halfWidth / sampleSize) >= reqWidth) {
                    sampleSize *= 2
                }
            }

            val decodeOpts = BitmapFactory.Options().apply {
                inSampleSize = sampleSize
                inPreferredConfig = Bitmap.Config.ARGB_8888
            }

            val bitmap = BitmapFactory.decodeFile(imagePath, decodeOpts)
                ?: throw IllegalStateException("Failed to decode bitmap from $imagePath")

            val wallpaperManager = WallpaperManager.getInstance(context)

            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N) {
                val flags = when (target) {
                    WallpaperTarget.SYSTEM -> WallpaperManager.FLAG_SYSTEM
                    WallpaperTarget.LOCK -> WallpaperManager.FLAG_LOCK
                    WallpaperTarget.BOTH -> WallpaperManager.FLAG_SYSTEM or WallpaperManager.FLAG_LOCK
                }
                wallpaperManager.setBitmap(bitmap, null, true, flags)
            } else {
                wallpaperManager.setBitmap(bitmap)
            }
            bitmap.recycle()
        }
    }
}
