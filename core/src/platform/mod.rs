//! Platform abstraction layer for device-specific functionality

use crate::Result;
use std::path::PathBuf;

/// Platform-specific storage interface
pub trait PlatformStorage {
    /// Get the application data directory
    fn get_app_data_dir(&self) -> Result<PathBuf>;
    
    /// Get the user documents directory
    fn get_documents_dir(&self) -> Result<PathBuf>;
    
    /// Get the temporary directory
    fn get_temp_dir(&self) -> Result<PathBuf>;
    
    /// Check if a path exists and is accessible
    fn path_exists(&self, path: &PathBuf) -> bool;
    
    /// Create a directory if it doesn't exist
    fn create_dir_all(&self, path: &PathBuf) -> Result<()>;
    
    /// Read file contents
    fn read_file(&self, path: &PathBuf) -> Result<Vec<u8>>;
    
    /// Write file contents
    fn write_file(&self, path: &PathBuf, contents: &[u8]) -> Result<()>;
    
    /// List files in a directory
    fn list_files(&self, path: &PathBuf) -> Result<Vec<PathBuf>>;
    
    /// Delete a file
    fn delete_file(&self, path: &PathBuf) -> Result<()>;
}

/// Platform-specific UI interface
pub trait PlatformUI {
    /// Show a file picker dialog
    fn show_file_picker(&self, filters: &[FileFilter]) -> Result<Option<PathBuf>>;
    
    /// Show a save file dialog
    fn show_save_dialog(&self, default_name: &str, filters: &[FileFilter]) -> Result<Option<PathBuf>>;
    
    /// Show a message dialog
    fn show_message(&self, title: &str, message: &str, message_type: MessageType) -> Result<()>;
    
    /// Show a confirmation dialog
    fn show_confirmation(&self, title: &str, message: &str) -> Result<bool>;
    
    /// Show a progress dialog
    fn show_progress(&self, title: &str, message: &str) -> Result<Box<dyn ProgressDialog>>;
    
    /// Get screen dimensions
    fn get_screen_size(&self) -> (u32, u32);
    
    /// Get DPI scaling factor
    fn get_dpi_scale(&self) -> f32;
}

/// File filter for dialogs
#[derive(Debug, Clone)]
pub struct FileFilter {
    pub name: String,
    pub extensions: Vec<String>,
}

/// Message types for dialogs
#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    Info,
    Warning,
    Error,
}

/// Progress dialog interface
pub trait ProgressDialog {
    /// Update progress (0.0 to 1.0)
    fn set_progress(&mut self, progress: f32);
    
    /// Update message
    fn set_message(&mut self, message: &str);
    
    /// Check if user cancelled
    fn is_cancelled(&self) -> bool;
    
    /// Close the dialog
    fn close(&mut self);
}

/// Platform-specific graphics interface
pub trait PlatformGraphics {
    /// Create a graphics context
    fn create_context(&self, width: u32, height: u32) -> Result<Box<dyn GraphicsContext>>;
    
    /// Get supported graphics APIs
    fn supported_apis(&self) -> Vec<GraphicsApi>;
}

/// Graphics context interface
pub trait GraphicsContext {
    /// Make this context current
    fn make_current(&mut self) -> Result<()>;
    
    /// Swap buffers (for double buffering)
    fn swap_buffers(&mut self) -> Result<()>;
    
    /// Resize the context
    fn resize(&mut self, width: u32, height: u32) -> Result<()>;
    
    /// Get the current size
    fn size(&self) -> (u32, u32);
}

/// Supported graphics APIs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsApi {
    OpenGL,
    Vulkan,
    DirectX11,
    DirectX12,
    Metal,
}

/// Platform capabilities
#[derive(Debug, Clone)]
pub struct PlatformCapabilities {
    pub platform_name: String,
    pub version: String,
    pub supports_file_system: bool,
    pub supports_networking: bool,
    pub supports_clipboard: bool,
    pub supports_notifications: bool,
    pub graphics_apis: Vec<GraphicsApi>,
    pub max_texture_size: u32,
    pub memory_limit: Option<u64>,
}

/// Main platform interface
pub trait Platform: Send + Sync {
    /// Get platform capabilities
    fn capabilities(&self) -> PlatformCapabilities;
    
    /// Get storage interface
    fn storage(&self) -> &dyn PlatformStorage;
    
    /// Get UI interface
    fn ui(&self) -> &dyn PlatformUI;
    
    /// Get graphics interface
    fn graphics(&self) -> &dyn PlatformGraphics;
    
    /// Initialize platform-specific resources
    fn initialize(&mut self) -> Result<()>;
    
    /// Cleanup platform-specific resources
    fn cleanup(&mut self) -> Result<()>;
}

/// Platform factory for creating platform-specific implementations
pub struct PlatformFactory;

impl PlatformFactory {
    /// Create a platform instance for the current system
    pub fn create() -> Result<Box<dyn Platform>> {
        #[cfg(target_os = "windows")]
        {
            Ok(Box::new(crate::platform::windows::WindowsPlatform::new()?))
        }
        
        #[cfg(target_os = "ios")]
        {
            Ok(Box::new(crate::platform::ios::IosPlatform::new()?))
        }
        
        #[cfg(target_os = "android")]
        {
            Ok(Box::new(crate::platform::android::AndroidPlatform::new()?))
        }
        
        #[cfg(not(any(target_os = "windows", target_os = "ios", target_os = "android")))]
        {
            Err(crate::WorldFoundryError::Platform(
                "Unsupported platform".to_string()
            ))
        }
    }
}

// Platform-specific modules (to be implemented)
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "ios")]
pub mod ios;

#[cfg(target_os = "android")]
pub mod android;