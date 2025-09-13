use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::sync::Arc;
use std::collections::HashMap;

use crate::data::WorldMap;
use crate::import::azgaar::AzgaarImporter;
use crate::rendering::tile::{TileRenderer, TileCoord};
use crate::export::png::PngExporter;

/// Error codes for the C API
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WfError {
    Ok = 0,
    InvalidArg = 1,
    Io = 2,
    Parse = 3,
    Internal = 255,
}

/// Opaque context structure
pub struct WfContext {
    _private: (),
}

/// Opaque project structure
pub struct WfProject {
    data: Arc<WorldMap>,
    path: String,
}

/// Opaque render structure
pub struct WfRender {
    renderer: TileRenderer,
    project: Arc<WorldMap>,
    tile_cache: HashMap<TileCoord, Vec<u8>>,
}

/// Convert Rust Result to C error code
fn result_to_error<T>(result: anyhow::Result<T>) -> (WfError, Option<T>) {
    match result {
        Ok(value) => (WfError::Ok, Some(value)),
        Err(err) => {
            let error_code = if err.to_string().contains("Invalid argument") {
                WfError::InvalidArg
            } else if err.to_string().contains("IO") || err.to_string().contains("file") {
                WfError::Io
            } else if err.to_string().contains("parse") || err.to_string().contains("JSON") {
                WfError::Parse
            } else {
                WfError::Internal
            };
            eprintln!("WorldFoundry Error: {}", err);
            (error_code, None)
        }
    }
}

/// Get the version string
#[no_mangle]
pub extern "C" fn wf_version() -> *const c_char {
    static VERSION: &str = "0.1.0\0";
    VERSION.as_ptr() as *const c_char
}

/// Create a new context
#[no_mangle]
pub extern "C" fn wf_create(out: *mut *mut WfContext) -> WfError {
    if out.is_null() {
        return WfError::InvalidArg;
    }

    let ctx = Box::new(WfContext { _private: () });
    unsafe {
        *out = Box::into_raw(ctx);
    }
    WfError::Ok
}

/// Destroy a context
#[no_mangle]
pub extern "C" fn wf_destroy(ctx: *mut WfContext) {
    if !ctx.is_null() {
        unsafe {
            let _ = Box::from_raw(ctx);
        }
    }
}

/// Open a project from a file path
#[no_mangle]
pub extern "C" fn wf_project_open(
    ctx: *mut WfContext,
    path: *const c_char,
    out: *mut *mut WfProject,
) -> WfError {
    if ctx.is_null() || path.is_null() || out.is_null() {
        return WfError::InvalidArg;
    }

    let path_str = unsafe {
        match CStr::from_ptr(path).to_str() {
            Ok(s) => s,
            Err(_) => return WfError::InvalidArg,
        }
    };

    let result = std::panic::catch_unwind(|| {
        let importer = AzgaarImporter::new();
        importer.import_from_file(path_str)
    });

    let world_data = match result {
        Ok(Ok(data)) => data,
        Ok(Err(err)) => {
            eprintln!("Import error: {}", err);
            return WfError::Parse;
        }
        Err(_) => {
            eprintln!("Panic during import");
            return WfError::Internal;
        }
    };

    let project = Box::new(WfProject {
        data: Arc::new(world_data),
        path: path_str.to_string(),
    });

    unsafe {
        *out = Box::into_raw(project);
    }
    WfError::Ok
}

/// Close a project
#[no_mangle]
pub extern "C" fn wf_project_close(project: *mut WfProject) {
    if !project.is_null() {
        unsafe {
            let _ = Box::from_raw(project);
        }
    }
}

/// Create a renderer for a project
#[no_mangle]
pub extern "C" fn wf_render_create(
    ctx: *mut WfContext,
    project: *mut WfProject,
    out: *mut *mut WfRender,
) -> WfError {
    if ctx.is_null() || project.is_null() || out.is_null() {
        return WfError::InvalidArg;
    }

    let project_ref = unsafe { &*project };
    
    let result = std::panic::catch_unwind(|| {
        TileRenderer::new(project_ref.data.clone())
    });

    let renderer = match result {
        Ok(Ok(r)) => r,
        Ok(Err(err)) => {
            eprintln!("Renderer creation error: {}", err);
            return WfError::Internal;
        }
        Err(_) => {
            eprintln!("Panic during renderer creation");
            return WfError::Internal;
        }
    };

    let render = Box::new(WfRender {
        renderer,
        project: project_ref.data.clone(),
        tile_cache: HashMap::new(),
    });

    unsafe {
        *out = Box::into_raw(render);
    }
    WfError::Ok
}

/// Destroy a renderer
#[no_mangle]
pub extern "C" fn wf_render_destroy(render: *mut WfRender) {
    if !render.is_null() {
        unsafe {
            let _ = Box::from_raw(render);
        }
    }
}

/// Render a tile
#[no_mangle]
pub extern "C" fn wf_render_tile(
    render: *mut WfRender,
    zoom: c_int,
    tx: c_int,
    ty: c_int,
    out_rgba: *mut u8,
    stride: c_int,
    w: c_int,
    h: c_int,
) -> WfError {
    if render.is_null() || out_rgba.is_null() {
        return WfError::InvalidArg;
    }

    if w != 256 || h != 256 || stride != 256 * 4 {
        return WfError::InvalidArg; // Only support 256x256 RGBA tiles for now
    }

    let render_ref = unsafe { &mut *render };
    let tile_coord = TileCoord {
        zoom: zoom as u32,
        x: tx as u32,
        y: ty as u32,
    };

    // Check cache first
    if let Some(cached_data) = render_ref.tile_cache.get(&tile_coord) {
        unsafe {
            ptr::copy_nonoverlapping(
                cached_data.as_ptr(),
                out_rgba,
                cached_data.len(),
            );
        }
        return WfError::Ok;
    }

    // Render the tile
    let result = std::panic::catch_unwind(|| {
        render_ref.renderer.render_tile(&tile_coord)
    });

    let tile_data = match result {
        Ok(Ok(data)) => data,
        Ok(Err(err)) => {
            eprintln!("Tile render error: {}", err);
            return WfError::Internal;
        }
        Err(_) => {
            eprintln!("Panic during tile rendering");
            return WfError::Internal;
        }
    };

    // Copy to output buffer
    unsafe {
        ptr::copy_nonoverlapping(
            tile_data.as_ptr(),
            out_rgba,
            tile_data.len().min(w as usize * h as usize * 4),
        );
    }

    // Cache the tile
    render_ref.tile_cache.insert(tile_coord, tile_data);

    WfError::Ok
}

/// Export a PNG
#[no_mangle]
pub extern "C" fn wf_export_png(
    project: *mut WfProject,
    out_path: *const c_char,
    width: c_int,
    height: c_int,
    style_json: *const c_char,
) -> WfError {
    if project.is_null() || out_path.is_null() {
        return WfError::InvalidArg;
    }

    let project_ref = unsafe { &*project };
    
    let out_path_str = unsafe {
        match CStr::from_ptr(out_path).to_str() {
            Ok(s) => s,
            Err(_) => return WfError::InvalidArg,
        }
    };

    let style_str = if style_json.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(style_json).to_str() {
                Ok(s) => Some(s),
                Err(_) => return WfError::InvalidArg,
            }
        }
    };

    let result = std::panic::catch_unwind(|| {
        let exporter = PngExporter::new();
        exporter.export(
            &project_ref.data,
            out_path_str,
            width as u32,
            height as u32,
            style_str,
        )
    });

    match result {
        Ok(Ok(_)) => WfError::Ok,
        Ok(Err(err)) => {
            eprintln!("Export error: {}", err);
            WfError::Io
        }
        Err(_) => {
            eprintln!("Panic during export");
            WfError::Internal
        }
    }
}