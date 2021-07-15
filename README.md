# include_bytes_aligned [![Crates.io](https://img.shields.io/crates/v/include_bytes_aligned)](https://crates.io/crates/include_bytes_aligned) [![docs.rs](https://img.shields.io/docsrs/include_bytes_aligned)](https://docs.rs/include_bytes_aligned/)

A simple macro that embeds the bytes of an external file into the executable and
guarantees that they are aligned.

# Usage

```rust
include_bytes_aligned!(ALIGNMENT, PATH)
```

Where `ALIGNMENT` is any integer literal (must be a power of 2), and PATH is a string literal path
to the file to include, just as in [`include_bytes!`](https://doc.rust-lang.org/nightly/core/macro.include_bytes.html).

# Examples

```
use include_bytes_aligned::include_bytes_aligned;

// Aligns the data to 16 bytes
static DATA: &'static [u8] = include_bytes_aligned!(16, "path/to/file.txt");
```

# Efficiency

This macro does not copy the bytes or duplicate them. Takes up the same amount of space in memory
as the usual [`include_bytes!`](https://doc.rust-lang.org/nightly/core/macro.include_bytes.html).