//! Rust types for ASAM OpenDRIVE 1.9 XML nodes.
//!
//! Enable the `serde` feature for XML-aware `Serialize` and `Deserialize`
//! implementations. Attributes use the `@name` convention understood by
//! `quick-xml`.
#![forbid(unsafe_code)]

mod generated;

pub use generated::*;
