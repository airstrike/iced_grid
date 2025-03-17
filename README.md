<div align="center">

# iced_grid

[![Made with iced](https://iced.rs/badge.svg)](https://github.com/iced-rs/iced)

A semi-clever, semi-cursed implementation of a grid layout widget for `iced` that arranges items in equal-sized cells.

*Void where prohibited. Use at your own risk. Side effects may include prettier UIs, organized layouts, and the occasional "how did they do that?" from your colleagues.*

</div>

## About

This crate provides a simple grid layout widget for the [iced](https://github.com/iced-rs/iced) GUI library. It allows you to arrange widgets in a grid with equal-sized cells, with customizable spacing and padding.

The implementation uses `iced`'s responsive layout capabilities to dynamically
size each cell based on the available space. Since `grid` just turns into a
`container()` (containing a `column()` of `row()`s), you can use "all" the usual
container methods like `spacing()`, `padding()`, `width()`, `height()`, etc.

If you have the need for additional container methods, feel free to open an
issue or PR!

## Usage

The crate provides two ways to create a grid:

1. Using the `grid` function:
```rust
use iced_grid::grid;

let items = (0..9).map(|i| text(format!("Item {}", i)));
let my_grid = grid(3, items).spacing(10.0).padding(20);
```

2. Using the `GridExt` trait extension on iterators:
```rust
use iced_grid::GridExt;

let items = (0..9).map(|i| text(format!("Item {}", i)));
let my_grid = items.grid(3).spacing(10.0).padding(20);
```

How easy is that?? 🎉

## Examples

Check out the `examples` directory for different usage examples or run them via:

```bash
# Basic example with colorful cells
$ cargo run --example simple

# More comprehensive overview
$ cargo run --example overview

# Calendar implementation using grid
$ cargo run --example calendar
```

The examples demonstrate:
- Basic grid layout with different styling options
- Using the grid for practical applications like calendars
- Styling grid cells with colors and containers

## License

MIT