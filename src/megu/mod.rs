mod script;
mod drop;
mod namespace;
mod extension;

pub use script::{MeguScript, ReadError};
pub use drop::{MeguDrop, DropType, DropFormat};
pub use namespace::{Namespace, DecodeError};
pub use extension::{Extension, ExtensionError};

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