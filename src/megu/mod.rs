mod script;
mod drop;
mod namespace;

pub use script::{MeguScript, ReadError};
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
		"entities/creeper" => Extension::Creeper,
		_ => return Err(ExtensionError::NotFound(value))
	};

	Ok(result)
}

#[derive(Debug, PartialEq)]
pub enum ExtensionError {
	DecodeError(DecodeError),
	NotFound(String)
}

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


use std::path::PathBuf;
type MeguResult<T> = Result<T, MeguError>;

pub fn interpret_file(path: impl Into<PathBuf>) -> MeguResult<MeguScript> {
	let path: PathBuf = path.into();
	if !path.exists() {
		return Err(MeguError::NotExist(path));
	}
	
	if path.is_dir() {
		return Err(MeguError::NotAFile(path));
	}

	let result = match MeguScript::from_path(&path) {
		Ok(result) => result,
		Err(error) => return Err(MeguError::Read((path, error)))
	};

	Ok(result)
}

#[derive(Debug)]
pub enum MeguError {
	NotExist(PathBuf),
	NotAFile(PathBuf),
	Read((PathBuf, ReadError))
}

impl From<(PathBuf, ReadError)> for MeguError {
	fn from(error: (PathBuf, ReadError)) -> MeguError {
		MeguError::Read(error)
	}
}

use std::fmt;
use colored::*;
impl fmt::Display for MeguError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			MeguError::NotExist(path) => write!(f, "'{}' does not exists", path.display().to_string().cyan()),
			MeguError::NotAFile(path) => write!(f, "'{}' is not a file", path.display().to_string().cyan()),
			MeguError::Read((path, error)) => write!(f, "[{}] {}", path.display().to_string().green(), error)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[should_panic]
	fn try_interpret_non_existence_file() {
		let path = PathBuf::from("/this/path/should/never/exists/EVER");
		interpret_file(&path).unwrap();
	}
}