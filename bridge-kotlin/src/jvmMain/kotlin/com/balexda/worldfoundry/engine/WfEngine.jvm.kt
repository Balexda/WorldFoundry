package com.balexda.worldfoundry.engine

import java.io.File
import java.nio.file.Paths

/**
 * JVM implementation of WfEngine using JNI to call the Rust C API.
 */
actual class WfEngine actual constructor() {
    
    companion object {
        private var isLibraryLoaded = false
        
        init {
            loadNativeLibrary()
        }
        
        private fun loadNativeLibrary() {
            if (isLibraryLoaded) return
            
            try {
                // Try to load from system library path first
                System.loadLibrary("world_foundry")
                isLibraryLoaded = true
            } catch (e: UnsatisfiedLinkError) {
                // Try to load from resources or local path
                try {
                    val libraryName = when {
                        System.getProperty("os.name").lowercase().contains("windows") -> "world_foundry.dll"
                        System.getProperty("os.name").lowercase().contains("mac") -> "libworld_foundry.dylib"
                        else -> "libworld_foundry.so"
                    }
                    
                    // Look for library in project directory
                    val projectDir = System.getProperty("user.dir")
                    val libraryPath = Paths.get(projectDir, "core", "target", "debug", libraryName).toString()
                    
                    if (File(libraryPath).exists()) {
                        System.load(libraryPath)
                        isLibraryLoaded = true
                    } else {
                        throw UnsatisfiedLinkError("Could not find native library: $libraryPath")
                    }
                } catch (e2: Exception) {
                    throw UnsatisfiedLinkError("Failed to load World Foundry native library: ${e2.message}")
                }
            }
        }
    }
    
    // Native method declarations
    private external fun nativeVersion(): String
    private external fun nativeIsInitialized(): Boolean
    private external fun nativeCreateContext(): Long
    private external fun nativeDestroyContext(contextPtr: Long)
    private external fun nativeCreateProject(contextPtr: Long, filePath: String): Long
    private external fun nativeDestroyProject(projectPtr: Long)
    private external fun nativeCreateRenderer(projectPtr: Long): Long
    private external fun nativeDestroyRenderer(rendererPtr: Long)
    private external fun nativeRenderTile(rendererPtr: Long, x: Int, y: Int, zoom: Int): ByteArray?
    private external fun nativeExportPng(projectPtr: Long, outputPath: String, width: Int, height: Int): Boolean
    
    actual fun version(): String {
        return if (isLibraryLoaded) {
            try {
                nativeVersion()
            } catch (e: Exception) {
                "0.1.0-fallback"
            }
        } else {
            "0.1.0-no-native"
        }
    }
    
    actual fun isInitialized(): Boolean {
        return if (isLibraryLoaded) {
            try {
                nativeIsInitialized()
            } catch (e: Exception) {
                false
            }
        } else {
            false
        }
    }
    
    actual fun getEngineInfo(): EngineInfo {
        return EngineInfo(
            version = version(),
            buildDate = "2025-09-12",
            features = if (isLibraryLoaded) {
                listOf("import", "render", "export", "tiles", "azgaar")
            } else {
                listOf("fallback")
            }
        )
    }
    
    actual fun createContext(): WfContext? {
        return if (isLibraryLoaded) {
            try {
                val ptr = nativeCreateContext()
                if (ptr != 0L) WfContextImpl(ptr) else null
            } catch (e: Exception) {
                null
            }
        } else {
            null
        }
    }
    
    actual fun destroyContext(context: WfContext) {
        if (isLibraryLoaded && context is WfContextImpl) {
            try {
                nativeDestroyContext(context.ptr)
            } catch (e: Exception) {
                // Log error but don't throw
            }
        }
    }
    
    actual fun createProject(context: WfContext, filePath: String): WfProject? {
        return if (isLibraryLoaded && context is WfContextImpl) {
            try {
                val ptr = nativeCreateProject(context.ptr, filePath)
                if (ptr != 0L) WfProjectImpl(ptr) else null
            } catch (e: Exception) {
                null
            }
        } else {
            null
        }
    }
    
    actual fun destroyProject(project: WfProject) {
        if (isLibraryLoaded && project is WfProjectImpl) {
            try {
                nativeDestroyProject(project.ptr)
            } catch (e: Exception) {
                // Log error but don't throw
            }
        }
    }
    
    actual fun createRenderer(project: WfProject): WfRenderer? {
        return if (isLibraryLoaded && project is WfProjectImpl) {
            try {
                val ptr = nativeCreateRenderer(project.ptr)
                if (ptr != 0L) WfRendererImpl(ptr) else null
            } catch (e: Exception) {
                null
            }
        } else {
            null
        }
    }
    
    actual fun destroyRenderer(renderer: WfRenderer) {
        if (isLibraryLoaded && renderer is WfRendererImpl) {
            try {
                nativeDestroyRenderer(renderer.ptr)
            } catch (e: Exception) {
                // Log error but don't throw
            }
        }
    }
    
    actual fun renderTile(renderer: WfRenderer, x: Int, y: Int, zoom: Int): ByteArray? {
        return if (isLibraryLoaded && renderer is WfRendererImpl) {
            try {
                nativeRenderTile(renderer.ptr, x, y, zoom)
            } catch (e: Exception) {
                null
            }
        } else {
            null
        }
    }
    
    actual fun exportPng(project: WfProject, outputPath: String, width: Int, height: Int): Boolean {
        return if (isLibraryLoaded && project is WfProjectImpl) {
            try {
                nativeExportPng(project.ptr, outputPath, width, height)
            } catch (e: Exception) {
                false
            }
        } else {
            false
        }
    }
}

// JVM implementations of opaque handles
actual open class WfContext
internal class WfContextImpl(val ptr: Long) : WfContext()

actual open class WfProject
internal class WfProjectImpl(val ptr: Long) : WfProject()

actual open class WfRenderer
internal class WfRendererImpl(val ptr: Long) : WfRenderer()