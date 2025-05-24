# Orbit Examples

This directory contains example Orbit applications demonstrating various features and patterns of the OrbitRS SDK.

## Rust Example Applications

### Component Examples
- `props_minimal.rs` - Minimal example showing props system usage
- `props_example.rs` - Advanced props system demonstration with validation
- `props_and_events.rs` - Example demonstrating props and event handling
- `component_lifecycle.rs` - Example of component lifecycle management with reactive state

### Rendering Examples
- `advanced_skia.rs` - Advanced example using Skia for custom rendering
- `wgpu_renderer.rs` - Example demonstrating WGPU renderer with 3D content
- `skia_test.rs` - Test program for the orbit window system with Skia rendering
- `window_test.rs` - Basic window system test

## Orbit File Format Examples
- `counter.orbit` - Basic counter with increment, decrement, and reset functionality
- `user-profile.orbit` - Profile editor with form validation and theme support

## Project Structure

The examples project is organized as follows:
- `src/` - Contains Rust example files
- `Cargo.toml` - Example project configuration
- `lib.rs` - Placeholder library file for project organization

## Running Examples

You can run any example using cargo:

```bash
# For Rust examples
cargo run --example props_minimal
cargo run --example advanced_skia
cargo run --example wgpu_renderer

# For .orbit files (requires orbiton CLI tool)
orbiton run counter.orbit
```
