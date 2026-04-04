# Miniquad Test Suite

A collection of small test programs for the [miniquad](https://github.com/not-fl3/miniquad) graphics engine.

## Running Tests

Each test is a standalone binary executable.

### Available Tests

| Binary | Description |
|--------|-------------|
| `window` | Tests basic window creation and lifecycle management. Verifies that a window can be created, displayed, and properly responds to close events. |
| `input` | Tests keyboard and mouse input handling. Verifies that key presses, mouse movement, and clicks are detected and processed correctly. |
| `graphics` | Tests basic graphics rendering. Draws simple shapes to verify that the rendering pipeline works correctly. |
| `mouse` | Tests mouse input by drawing white pixels while the left mouse button is held down and moved across the window. |
| `wireframe` | Renders a rotating 3D wireframe cube using `PrimitiveType::Lines`. You should see a green wireframe cube rotating on a black background. |
| `starfield` | Renders an animated starfield using `PrimitiveType::Points` with dynamic vertex buffer updates (`BufferUsage::Stream`). You should see white stars streaming toward you on a dark background. |
| `viewport` | Renders a colour-shaded triangle inside a restricted viewport centred in the window using `apply_viewport`. A light grey rectangle is drawn around the viewport border so its bounds are clearly visible. |

Run any test with:

```bash
cargo run --bin <binary_name>
```

## Building All Tests

To build all test programs at once:

```bash
cargo build --bins
```

## Testing Against Different Miniquad Versions

Edit `Cargo.toml` to change the miniquad dependency. Options:

- **Main branch:** `branch = "main"`
- **Specific commit:** `rev = "abc1234def567"`
- **Specific version from crates.io:** `version = "0.4"`
- **Local path:** `path = "/path/to/miniquad"`

After changing the dependency, run:

```bash
cargo clean
cargo build --bins
```

## Project Structure

```
miniquad-tests/
├── Cargo.toml              # Package manifest with miniquad dependency
├── README.md               # This file
├── .gitignore              # Git ignore rules
├── src/
│   ├── lib.rs              # Shared test utilities and helpers
│   └── bin/
│       ├── window.rs       # Window creation and lifecycle test
│       ├── input.rs        # Input handling test
│       ├── graphics.rs     # Graphics rendering test
│       ├── mouse.rs        # Mouse drawing test
│       ├── wireframe.rs    # Rotating 3D wireframe cube test
│       ├── starfield.rs    # Animated starfield test
│       └── viewport.rs     # Shaded triangle in a centred sub-viewport test
```

## Adding New Tests

1. Create a new `.rs` file in `src/bin/`, e.g., `src/bin/my_test.rs`
2. Add a `[[bin]]` entry in `Cargo.toml` pointing to the new file
3. Implement your test using the shared utilities from `src/lib.rs`
4. Run with `cargo run --bin my_test`

## Shared Utilities

Common functions and helpers are defined in `src/lib.rs` to avoid code duplication. Import them in your test binaries:

```rust
use miniquad_test_utils::*;
```

## Requirements

- Rust 1.70+ (check your version with `rustc --version`)
- Platform-specific miniquad dependencies (see [miniquad docs](https://docs.rs/miniquad/latest/miniquad/))

## Troubleshooting

**Build fails with dependency errors:**
Ensure you have the latest Rust toolchain:
```bash
rustup update
```

**Test window doesn't appear:**
Check your graphics drivers and ensure your platform supports the graphics backend (Metal on macOS, DirectX on Windows, GL on Linux).

**Cargo.lock conflicts:**
The `Cargo.lock` file is committed to ensure reproducible builds. If you're testing different miniquad branches, run:
```bash
cargo update
```

## License

These test programs are provided as examples for testing miniquad. See the miniquad repository for its license.
