use super::{Namespace, DecodeError, MeguScript, ReadError};
use std::path::PathBuf;

/// Extension Script of MeguScript
/// 
/// `Extension` will follow /loot command's path convention.  
/// To refer to `creeper` loot table use `minecraft:entities/creeper`
/// 
/// # Examples
/// ```
/// # use rna::Extension;
/// let creeper_extend = Extension::get_extension("minecraft:entities/creeper").unwrap();
/// ```
/// If the input string is not a vanilla's path it will panic
/// ```should_panic
/// # use rna::Extension;
/// let should_panic = Extension::get_extension("this/path/does/not/exists").unwrap();
/// ```
#[derive(Clone, PartialEq, Eq, Debug, PartialOrd)]
pub struct Extension {
	location: PathBuf
}

impl Extension {
	pub fn new(location: impl Into<PathBuf>) -> Extension {
		let location = location.into();
		Extension { location }
	}

	/// Get extension from given Namespace
	/// 
	/// # Setting up
	/// You need to include the source files for the loot table database yourself.
	/// 
	/// Which you need to place the file in this order: `{base_path}/{prefix}/{suffix}.ult` where `prefix` and `suffix` refer to Namespace
	pub fn get_extension(value: impl Into<String>, base_path: impl Into<PathBuf>) -> Result<Extension, ExtensionError> {
		let value = value.into();
		let namespace = Namespace::decode(&value)?;
	
		let path: PathBuf = base_path.into();
		let path = path
			.join(namespace.prefix)
			.join(format!("{}.ult", namespace.suffix));
	
		if !path.exists() {
			return Err(ExtensionError::NotFound(value));
		}
	
		Ok(Extension::new(path))
	}

	/// Create MeguScript from this Extension
	pub fn compile(&self, base_path: impl Into<PathBuf>) -> Result<MeguScript, ReadError> {
		MeguScript::from_path(&self.location, base_path)
	}
}

/// General error type for `get_extension()` function.
#[derive(Debug, PartialEq)]
pub enum ExtensionError {
	/// Emit when Namespace is invalid
	DecodeError(DecodeError),
	/// Emit when cannot find extension with that name
	NotFound(String)
}

use std::fmt;
use colored::*;
impl From<DecodeError> for ExtensionError {
	fn from(error: DecodeError) -> ExtensionError {
		ExtensionError::DecodeError(error)
	}
}
impl fmt::Display for ExtensionError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ExtensionError::DecodeError(error) => write!(f, "{}", error),
			ExtensionError::NotFound(value) => write!(f, "Does not recognized '{}' in {} field.", value.cyan(), "extend".black().on_white())
		}
	}
}
