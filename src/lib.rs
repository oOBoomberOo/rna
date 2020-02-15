//! This is a library for interpreting "Loot Table Script".
//! 
//! Loot Table Script is created by [Minecraft Datapacks](https://discord.gg/56ySADc) community to create more robust loot table syntax.
//! # Examples
//! ```
//! let loot_a = rna::interpret_file("test/loot_a.megu").unwrap();
//! let loot_b = rna::interpret_file("test/loot_b.megu").unwrap();
//! 
//! let merged_loot = rna::merge(&[loot_a, loot_b]).unwrap();
//! ```

mod megu;

pub use megu::{
	interpret_file,
	merge,
	MeguScript,
	MeguDrop,
	Namespace,
	Extension
};

/// Module containg every errors type in this crate
pub mod error {
	pub use crate::megu::{MeguError, MeguResult};
	pub use crate::megu::script::{ScriptFormatError, ReadError};
	pub use crate::megu::namespace::DecodeError;
	pub use crate::megu::extension::ExtensionError;
	pub use crate::megu::drop::DropTypeError;
}

/// MeguScript module
pub mod script {
	pub use crate::megu::script::{MeguScript, ScriptFormat, ScriptFormatError, ReadError};
}

/// Namespace module
pub mod namespace {
	pub use crate::megu::namespace::{Namespace, DecodeError};
}

/// Extension module
pub mod extension {
	pub use crate::megu::extension::{Extension, ExtensionError};
}

/// MeguDrop module
pub mod drop {
	pub use crate::megu::drop::{MeguDrop, DropType, DropFormat, DropTypeError};
}