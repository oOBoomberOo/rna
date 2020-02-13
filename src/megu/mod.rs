mod script;
mod drop;
mod namespace;

pub use script::MeguScript;
pub use drop::{MeguDrop, DropType, DropFormat};
pub use namespace::{Namespace, DecodeError};

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd)]
enum Extension {
	Creeper
}

fn get_extension(value: impl Into<String>) -> Result<Extension, ExtensionError> {
	let value = value.into();
	let namespace = Namespace::decode(&value)?;

	let result = match namespace.suffix.as_ref() {
		"creeper" => Extension::Creeper,
		_ => return Err(ExtensionError::NotFound(value))
	};

	Ok(result)
}

#[derive(Debug)]
enum ExtensionError {
	DecodeError(DecodeError),
	NotFound(String)
}

impl From<DecodeError> for ExtensionError {
	fn from(error: DecodeError) -> ExtensionError {
		ExtensionError::DecodeError(error)
	}
}



use std::path::PathBuf;
type MeguResult<T> = Result<T, MeguError>;
pub fn interpret_file(path: impl Into<PathBuf>) -> MeguResult<()> {
	let path: PathBuf = path.into();
	if !path.exists() {
		return Err(MeguError::NotExist(path));
	}
	
	if path.is_dir() {
		return Err(MeguError::NotAFile(path));
	}

	Ok(())
}

#[derive(Debug)]
pub enum MeguError {
	NotExist(PathBuf),
	NotAFile(PathBuf),
	InvalidFormat(PathBuf)
}

use std::fmt;
use colored::*;
impl fmt::Display for MeguError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			MeguError::NotExist(path) => write!(f, "'{}' does not exists", path.display().to_string().cyan()),
			MeguError::NotAFile(path) => write!(f, "'{}' is not a file", path.display().to_string().cyan()),
			MeguError::InvalidFormat(path) => write!(f, "'{}' contain invalid format", path.display().to_string().cyan())
		}
	}
}