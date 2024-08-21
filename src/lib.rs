#![no_std]
//! A simple macro that embeds the bytes of an external file into the executable and
//! guarantees that they are aligned.
//!
//! # Usage
//!
//! ```
//! include_bytes_aligned!(ALIGNMENT, PATH)
//! ```
//!
//! Where `ALIGNMENT` is any integer literal (must be a power of 2), and PATH is a string literal path
//! to the file to include, just as in [`include_bytes!`](std::include_bytes).
//!
//! # Examples
//!
//! ```
//! use include_bytes_aligned::include_bytes_aligned;
//!
//! // Aligns the data to 16 bytes
//! static DATA: &'static [u8] = include_bytes_aligned!(16, "path/to/file.txt");
//! ```
//!
//! # Efficiency
//!
//! This macro does not copy the bytes or duplicate them. Takes up the same amount of space in memory
//! as the usual [`include_bytes!`](std::include_bytes).

#[macro_export]
macro_rules! include_bytes_aligned {
    ($align_to:expr, $path:expr) => {{
        #[repr(C, align($align_to))]
        struct __Aligned<T: ?Sized>(T);

        const __DATA: &'static __Aligned<[u8]> = &__Aligned(*include_bytes!($path));

        &__DATA.0
    }};
}
