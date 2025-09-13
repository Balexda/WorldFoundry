package com.balexda.worldfoundry.engine

/**
 * World Foundry engine facade that interfaces with the Rust core via C ABI.
 * Uses expect/actual declarations for platform-specific implementations.
 */
expect class WfEngine() {
    fun version(): String
    fun isInitialized(): Boolean
    fun getEngineInfo(): EngineInfo
    
    // Context management
    fun createContext(): WfContext?
    fun destroyContext(context: WfContext)
    
    // Project management
    fun createProject(context: WfContext, filePath: String): WfProject?
    fun destroyProject(project: WfProject)
    
    // Rendering
    fun createRenderer(project: WfProject): WfRenderer?
    fun destroyRenderer(renderer: WfRenderer)
    fun renderTile(renderer: WfRenderer, x: Int, y: Int, zoom: Int): ByteArray?
    
    // Export
    fun exportPng(project: WfProject, outputPath: String, width: Int, height: Int): Boolean
}

data class EngineInfo(
    val version: String,
    val buildDate: String,
    val features: List<String>
)

// Opaque handles for C API objects
expect class WfContext
expect class WfProject  
expect class WfRenderer