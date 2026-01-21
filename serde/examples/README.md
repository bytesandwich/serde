# Render Extraction Examples

This directory contains examples demonstrating different approaches to extracting
renderable types from game state structures.

## Running Examples

```bash
# Run the basic extraction example
cargo run --example render_extraction_basic

# Run the extractable trait example
cargo run --example render_extraction_trait
```

## Examples Overview

### 1. Basic Manual Extraction (`render_extraction_basic.rs`)

Shows the simplest approach using manual extraction with a wrapper type:
- Uses `ExtractWrapper` to manually walk the game state
- Explicitly calls `to_primitive()` on each renderable field
- Good for simple cases where you have full control

### 2. Extractable Trait (`render_extraction_trait.rs`)

Demonstrates the recommended approach using the `Extractable` trait:
- Uses `ExtractHelper` trait for automatic extraction
- Handles different types (single values, Vec, etc.) automatically
- Clean separation between game state and rendering primitives
- Zero runtime type detection overhead

### 3. Custom Serializer (in test file)

Shows a serde-based approach with custom serializer:
- Walks the type tree using serde's Serializer trait
- Tracks path through struct fields and array indices
- Cannot automatically detect Renderable types (Rust limitation)
- Useful for understanding serde internals

## Key Concepts

### Renderable Trait

Types that can be converted to rendering primitives implement this trait:

```rust
pub trait Renderable {
    fn to_primitive(&self) -> Primitive;
}
```

### Extractable Trait

Types that can extract renderable primitives from their fields:

```rust
pub trait Extractable {
    fn extract_renderables(&self) -> HashMap<String, Primitive>;
}
```

### ExtractHelper

Helper trait that provides extraction logic for different types:

```rust
pub trait ExtractHelper {
    fn extract(&self, field_name: &str, prefix: &str, result: &mut HashMap<String, Primitive>);
}
```

Implemented for:
- Any type T where T: Renderable (extracts single value)
- Vec<T> where T: Renderable (extracts with array indices)

## Recommendations

For production use in a 2D game engine:

1. **Use the Extractable trait pattern** - Best balance of simplicity and power
2. **Implement ExtractHelper for collection types** (Vec, HashMap, etc.)
3. **Consider a derive macro** for automatic Extractable implementation
4. **Don't use runtime type detection** - Rust doesn't support it efficiently

See `RENDER_EXTRACTION_RESEARCH.md` for detailed trade-off analysis.
