//! Read ESTree JSON into oxc's typed Rust AST.
//!
//! The codec works across a string boundary: the JS side serializes an ESTree
//! AST with `JSON.stringify`, and the Rust side reads that JSON string into a
//! typed [`oxc`](https://docs.rs/oxc) `Program`.
//!
//! ## Usage
//!
//! **JS → Rust**: read an ESTree JSON string into a typed AST:
//!
//! ```ts
//! const json: string = JSON.stringify(ast); // ESTree AST → JSON string
//! ```
//!
//! ```rust,ignore
//! use oxc::allocator::Allocator;
//! use oxc::ast::ast::{Program, SourceType};
//! use oxc_estree_codec::{JsonToProgramOptions, json_to_program};
//!
//! let json: &str = r#"{"type":"Program","body":[...],"sourceType":"module"}"#;
//!
//! let allocator: Allocator = Allocator::default();
//!
//! let source_text: &str = "const x = 1;";
//!
//! let program: Program<'_> = json_to_program(
//!     json,
//!     JsonToProgramOptions {
//!         allocator: &allocator,
//!         source_type: SourceType::from_path("index.js").unwrap(),
//!         source_text,
//!     },
//! )?;
//! ```

mod _types;
mod errors;
mod reader;

use oxc::ast::ast::Program;

use crate::reader::engine::program::ProgramReader;

pub use crate::_types::options::JsonToProgramOptions;
pub use crate::errors::read::ReadError;

/// Read an ESTree JSON string into a typed oxc `Program`.
pub fn json_to_program<'j, 'a>(
    json: &'j str,
    options: JsonToProgramOptions<'a>,
) -> Result<Program<'a>, ReadError> {
    let value: sonic_rs::Value =
        crate::reader::json::parse(json).map_err(ReadError::from_message)?;

    let reader: ProgramReader<'a> = ProgramReader::new(options.allocator);

    reader.read(&value, options.source_type, options.source_text)
}

#[doc(hidden)]
pub mod __internal {
    pub use crate::reader::engine::program::ProgramReader;
}
