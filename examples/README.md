# World Foundry Examples

This directory contains example code demonstrating how to use the World Foundry core engine.

## Running Examples

To run the basic usage example:

```bash
cd core
cargo run --example basic_usage
```

Or to run the CLI tool:

```bash
cd core
cargo run --bin world-foundry-cli -- --help
```

## Examples

### basic_usage.rs

Demonstrates the core functionality of World Foundry:

- Creating a new world map manually
- Importing maps from Azgaar format
- Exporting to various formats (JSON, PNG)
- Rendering maps with different styles
- Platform capability detection

### CLI Tool

The command-line interface provides access to core functionality:

```bash
# Import an Azgaar map
cargo run --bin world-foundry-cli import -i map.json -o world.json

# Generate a new world
cargo run --bin world-foundry-cli generate -o new_world.json --width 2048 --height 1024 --seed 42

# Render a world to an image
cargo run --bin world-foundry-cli render -i world.json -o world.png --width 2048 --height 1024 --style political

# Export to different formats
cargo run --bin world-foundry-cli export -i world.json -o world.png

# Show information about a map
cargo run --bin world-foundry-cli info world.json
```

## Building

Make sure you have Rust installed, then:

```bash
cd core
cargo build --release
```

## Dependencies

The examples require the same dependencies as the core engine. See `core/Cargo.toml` for the complete list.

## Platform-Specific Examples

Platform-specific examples will be added in their respective directories:

- `platforms/windows/examples/` - Windows WinUI 3 examples
- `platforms/ios/examples/` - iOS SwiftUI examples  
- `platforms/android/examples/` - Android Jetpack Compose examples

## Contributing

When adding new examples:

1. Add the example file to this directory
2. Update this README with a description
3. Ensure the example is well-documented
4. Test the example on multiple platforms if applicable