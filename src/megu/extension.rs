use super::{Namespace, DecodeError, MeguScript, ReadError};
use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, Debug, PartialOrd)]
pub struct Extension {
	pub location: PathBuf
}

impl Extension {
	pub fn new(location: impl Into<PathBuf>) -> Extension {
		let location = location.into();
		Extension { location }
	}

	pub fn get_extension(value: impl Into<String>) -> Result<Extension, ExtensionError> {
		let value = value.into();
		let namespace = Namespace::decode(&value)?;
	
		let path = PathBuf::from(
			format!("resource/{}/{}.megu", namespace.prefix, namespace.suffix)
		);
	
		if !path.exists() {
			return Err(ExtensionError::NotFound(value));
		}
	
		Ok(Extension::new(path))
	}

	pub fn compile(&self) -> Result<MeguScript, ReadError> {
		MeguScript::from_path(&self.location)
	}
}

#[derive(Debug, PartialEq)]
pub enum ExtensionError {
	DecodeError(DecodeError),
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
