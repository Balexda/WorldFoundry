package com.balexda.worldfoundry.engine

/**
 * Dummy engine facade for the splash milestone.
 * Later, this will be replaced with FFI calls to the Rust core via a stable C ABI.
 */
class WfEngine {
    fun version(): String = "0.0.0-splash"
    
    fun isInitialized(): Boolean = true
    
    fun getEngineInfo(): EngineInfo {
        return EngineInfo(
            version = version(),
            buildDate = "2025-09-12",
            features = listOf("splash", "dummy")
        )
    }
}

data class EngineInfo(
    val version: String,
    val buildDate: String,
    val features: List<String>
)