//! World Map Component - Core library
//!
//! This crate provides core data structures and utilities for rendering
//! interactive world maps with markers.

/// Error types
pub mod error;
/// Marker types and utilities
pub mod marker;
/// GPU marker buffer management
pub mod marker_buffer;
/// Map projection implementations
pub mod projection;
/// Visual theme configuration
pub mod theme;
/// World topology data structures
pub mod topology;

pub use error::CoreError;
