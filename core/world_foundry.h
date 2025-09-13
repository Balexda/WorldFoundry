#ifndef WORLD_FOUNDRY_H
#define WORLD_FOUNDRY_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stddef.h>

/**
 * World Foundry C API
 * 
 * This header provides the C API for the World Foundry core engine,
 * enabling cross-platform integration with Kotlin Multiplatform,
 * Swift, and other native applications.
 */

// Version information
const char* wf_version(void);

// Opaque types
typedef struct wf_ctx wf_ctx;
typedef struct wf_project wf_project;
typedef struct wf_render wf_render;

// Error codes
typedef enum {
    WF_OK = 0,
    WF_ERR_INVALID_ARG = 1,
    WF_ERR_IO = 2,
    WF_ERR_PARSE = 3,
    WF_ERR_INTERNAL = 255
} wf_err;

// Context management
wf_err wf_create(wf_ctx** out);
void wf_destroy(wf_ctx* ctx);

// Project management
wf_err wf_project_open(wf_ctx* ctx, const char* path, wf_project** out);
void wf_project_close(wf_project* project);

// Rendering
wf_err wf_render_create(wf_ctx* ctx, wf_project* project, wf_render** out);
void wf_render_destroy(wf_render* render);

/**
 * Render a tile at the specified zoom level and coordinates.
 * 
 * @param render The render context
 * @param zoom Zoom level (0 = most zoomed out)
 * @param tx Tile X coordinate
 * @param ty Tile Y coordinate
 * @param out_rgba Output buffer for RGBA data (must be pre-allocated)
 * @param stride Bytes per row (should be w * 4 for RGBA)
 * @param w Tile width in pixels (must be 256)
 * @param h Tile height in pixels (must be 256)
 * @return Error code
 */
wf_err wf_render_tile(
    wf_render* render,
    int zoom,
    int tx,
    int ty,
    uint8_t* out_rgba,
    int stride,
    int w,
    int h
);

// Export
/**
 * Export the project as a PNG image.
 * 
 * @param project The project to export
 * @param out_path Output file path (UTF-8 encoded)
 * @param width Image width in pixels
 * @param height Image height in pixels
 * @param style_json Optional JSON string for styling (can be NULL)
 * @return Error code
 */
wf_err wf_export_png(
    wf_project* project,
    const char* out_path,
    int width,
    int height,
    const char* style_json
);

#ifdef __cplusplus
}
#endif

#endif // WORLD_FOUNDRY_H