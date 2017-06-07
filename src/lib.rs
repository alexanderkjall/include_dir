//! The logical evolution of the `include_str!()` macro to allow embedding
//! entire file trees.
//!
//! If you want to use this in stable, you'll need a `build.rs` build script.
//!
//! ```rust,no_run
//! extern crate include_dir;
//!
//! use std::env;
//! use std::path::Path;
//! use include_dir::include_dir;
//!
//! fn main() {
//!     let outdir = env::var("OUT_DIR").unwrap();
//!     let dest_path = Path::new(&outdir).join("assets.rs");
//!
//!     include_dir("assets")
//!         .as_variable("SRC")
//!         .to_file(dest_path)
//!         .unwrap();
//!     }
//! ```
//!
//! Then in one of your source files, you'll need to include the generated
//! `assets.rs` file.
//!
//! ```rust,ignore
//! include!(concat!(env!("OUT_DIR"), "/assets.rs"));
//! ```

#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unused_import_braces, unused_qualifications)]

#[macro_use]
extern crate error_chain;
extern crate walkdir;

#[cfg(test)]
extern crate tempfile;
#[cfg(test)]
extern crate tempdir;

mod files;
mod dirs;
mod serializer;
mod frontend;

pub use errors::*;
pub use files::File;
pub use dirs::Dir;
pub use frontend::{include_dir, IncludeDirBuilder};


mod errors {
    error_chain!{
        foreign_links {
            IO(::std::io::Error) #[doc = "A wrapper around a std::io::Error"];
        }
    }
}
