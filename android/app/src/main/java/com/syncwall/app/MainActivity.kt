package com.syncwall.app

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Image
import androidx.compose.material.icons.filled.QrCode
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import com.syncwall.app.ui.screens.HomeScreen
import com.syncwall.app.ui.screens.PairScreen
import com.syncwall.app.ui.screens.SettingsScreen
import com.syncwall.app.ui.theme.Emerald400
import com.syncwall.app.ui.theme.Slate400
import com.syncwall.app.ui.theme.Slate800
import com.syncwall.app.ui.theme.Slate900
import com.syncwall.app.ui.theme.SyncWallTheme

sealed class Screen(val route: String, val title: String, val icon: ImageVector) {
    object Home : Screen("home", "Wallpapers", Icons.Default.Image)
    object Pair : Screen("pair", "Pair", Icons.Default.QrCode)
    object Settings : Screen("settings", "Settings", Icons.Default.Settings)
}

class MainActivity : ComponentActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            SyncWallTheme {
                val navController = rememberNavController()
                val items = listOf(Screen.Home, Screen.Pair, Screen.Settings)
                var currentRoute by remember { mutableStateOf(Screen.Home.route) }

                Scaffold(
                    modifier = Modifier.fillMaxSize(),
                    containerColor = Slate900,
                    bottomBar = {
                        NavigationBar(
                            containerColor = Slate800,
                            contentColor = Emerald400
                        ) {
                            items.forEach { screen ->
                                NavigationBarItem(
                                    icon = { Icon(screen.icon, contentDescription = screen.title) },
                                    label = { Text(screen.title) },
                                    selected = currentRoute == screen.route,
                                    onClick = {
                                        currentRoute = screen.route
                                        navController.navigate(screen.route) {
                                            popUpTo(navController.graph.startDestinationId) {
                                                saveState = true
                                            }
                                            launchSingleTop = true
                                            restoreState = true
                                        }
                                    },
                                    colors = NavigationBarItemDefaults.colors(
                                        selectedIconColor = Slate900,
                                        selectedTextColor = Emerald400,
                                        indicatorColor = Emerald400,
                                        unselectedIconColor = Slate400,
                                        unselectedTextColor = Slate400
                                    )
                                )
                            }
                        }
                    }
                ) { innerPadding ->
                    NavHost(
                        navController = navController,
                        startDestination = Screen.Home.route,
                        modifier = Modifier.padding(innerPadding)
                    ) {
                        composable(Screen.Home.route) {
                            HomeScreen()
                        }
                        composable(Screen.Pair.route) {
                            PairScreen()
                        }
                        composable(Screen.Settings.route) {
                            SettingsScreen()
                        }
                    }
                }
            }
        }
    }
}
