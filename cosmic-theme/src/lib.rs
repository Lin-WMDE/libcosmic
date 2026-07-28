#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]

//! Cosmic theme library.
//!
//! Provides utilities for creating custom cosmic themes.
//!

pub use model::*;

mod model;

#[cfg(feature = "export")]
mod output;

/// composite colors in srgb
pub mod composite;
/// get color steps
pub mod steps;

/// name of cosmic theme
pub const NAME: &str = "fun.wmde.Theme";

/// name of the WMDE icon theme (its directory under `/usr/share/icons`)
pub const ICON_THEME: &str = "WMDE";

/// name of the dark WMDE icon theme. Same art as [`ICON_THEME`]; it falls back to the
/// light-ink base set instead of the dark-ink one, for icons Qt asks for by plain name.
pub const ICON_THEME_DARK: &str = "WMDE-Dark";

pub use palette;
