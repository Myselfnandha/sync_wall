# UniFFI and JNA Proguard Rules
-keep class com.sun.jna.** { *; }
-keep class * implements com.sun.jna.** { *; }
-keep class uniffi.** { *; }
-keepclassmembers class * {
    native <methods>;
}
