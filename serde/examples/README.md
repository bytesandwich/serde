# Render Extraction Examples

This directory contains runnable examples demonstrating different approaches to extracting
renderable types from game state structures.

**📖 For complete documentation, see [RENDER_EXTRACTION_COMPLETE_GUIDE.md](../../RENDER_EXTRACTION_COMPLETE_GUIDE.md)**

## Quick Start

```bash
# Run the basic extraction example
cargo run --example render_extraction_basic

# Run the extractable trait example (recommended)
cargo run --example render_extraction_trait
```

## Examples

### 1. `render_extraction_basic.rs` - Manual Extraction

Simple approach using manual extraction:
- Explicitly calls `to_primitive()` on each renderable field
- Full control over extraction logic
- Good for learning the basics

### 2. `render_extraction_trait.rs` - Extractable Trait ⭐ Recommended

Production-ready approach using traits:
- Uses `ExtractHelper` trait for automatic handling
- Supports single values, vectors, and extensible to other collections
- Zero runtime type detection overhead
- Clean separation of concerns

## Key Takeaway

**Use the Extractable trait pattern** with a custom derive macro for production:

```rust
#[derive(Serialize, Extractable)]
pub struct SoccerGame {
    #[extract] pub field: Field,
    #[extract] pub ball: Ball,
    #[extract] pub players: Vec<Player>,
    pub score: [u32; 2],  // Not marked, won't extract
}
```

See the [complete guide](../../RENDER_EXTRACTION_COMPLETE_GUIDE.md) for detailed analysis of all approaches, implementation guide, and trade-off analysis.
