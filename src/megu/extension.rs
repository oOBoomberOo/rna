use super::{DecodeError, MeguScript};
// use std::path::PathBuf;
use crate::megu::script::{ScriptFormat, ScriptFormatError};

const DATABASE: &str = include_str!("../../resource/database.json");

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
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Extension {
	// location: PathBuf
	data: ScriptFormat
}

use serde_json as js;
use std::collections::HashMap;
impl Extension {
	// pub fn new(location: impl Into<PathBuf>) -> Extension {
	pub fn new(data: ScriptFormat) -> Extension {
		// let location = location.into();
		Extension {
			// location
			data
		}
	}

	/// Get extension from given Namespace
	pub fn get_extension(value: impl Into<String>) -> Result<Extension, ExtensionError> {
		let value = value.into();

		let data: HashMap<String, ScriptFormat> = js::from_str(DATABASE).unwrap();
	
		/*
		let path = PathBuf::from(
			format!("resource/{}/{}.megu", namespace.prefix, namespace.suffix)
		);
	
		if !path.exists() {
			return Err(ExtensionError::NotFound(value));
		}
		*/

		if data.get(&value).is_none() {
			return Err(ExtensionError::NotFound(value));
		}

		let data: ScriptFormat = data[&value].clone();
	
		// Ok(Extension::new(path))
		Ok(Extension::new(data))
	}

	/// Create MeguScript from this Extension
	pub fn compile(&self) -> Result<MeguScript, ScriptFormatError> {
		// MeguScript::from_path(&self.location)
		MeguScript::from_script_format(self.data.clone())
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
