pub(crate) mod script;
pub(crate) mod drop;
pub(crate) mod namespace;
pub(crate) mod extension;

pub use script::{MeguScript, ReadError};
pub use drop::{MeguDrop, DropType, DropFormat};
pub use namespace::{Namespace, DecodeError};
pub use extension::{Extension, ExtensionError};

use std::path::PathBuf;
/// Shorthand for defining a `Result` that can fail with `MeguError` type
pub type MeguResult<T> = Result<T, MeguError>;

/// Read and Interpret syntax from the given path.
/// 
/// # Errors
/// This method can fail when:
/// - File does not exists
/// - Path is a directory
/// - There is syntax error inside the file
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

/// General error type for `interpret_file()` function
#[derive(Debug)]
pub enum MeguError {
	/// Emit when `path` does not exists
	NotExist(PathBuf),
	/// Emit when `path` is not a file
	NotAFile(PathBuf),
	/// Emit when file contain syntax error
	Read((PathBuf, ReadError))
}

use std::fmt;
use colored::*;
use std::error::Error;
impl From<(PathBuf, ReadError)> for MeguError {
	fn from(error: (PathBuf, ReadError)) -> MeguError {
		MeguError::Read(error)
	}
}
impl Error for MeguError {}
impl fmt::Display for MeguError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			MeguError::NotExist(path) => write!(f, "'{}' does not exists", path.display().to_string().cyan()),
			MeguError::NotAFile(path) => write!(f, "'{}' is not a file", path.display().to_string().cyan()),
			MeguError::Read((path, error)) => write!(f, "[{}] {}", path.display().to_string().green(), error)
		}
	}
}

/// Merge MeguScripts together.
pub fn merge(scripts: &[MeguScript]) -> Result<MeguScript, Box<dyn Error>> {
	let mut result: MeguScript = MeguScript::default();

	for script in scripts {
		let script = script.compile()?;
		script.merge(&mut result);
	}

	result.remove_drops();

	Ok(result)
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