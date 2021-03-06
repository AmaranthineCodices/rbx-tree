//! Super early, unstable binary format (rbxm and rbxl) serializer and
//! deserializer for rbx-tree.
//!
//! Both the serializer and deserializer are functioning for limited property
//! types. `String` and `Bool` (from the `rbx_tree` crate) are the only
//! supported values. Unrecognized values will be ignored when deserializing,
//! and cause a panic when serializing.

mod core;
mod types;
mod serializer;
mod deserializer;

pub use crate::{
    serializer::{encode, EncodeError},
    deserializer::{decode, DecodeError},
};