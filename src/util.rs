use std::path::PathBuf;
use std::fs;
use serde_json as js;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct MetaFormat {
	compiler_options: Option<Vec<CompilerOption>>
}

#[derive(Serialize, Deserialize)]
struct CompilerOption {
	name: String
}

/// Shorthand for `Result<(), MetaError>`
pub type MetaResult = Result<(), MetaError>;
/// Check `pack.mcmeta` file for `compiler_options` field
pub fn check_meta(path: impl Into<PathBuf>) -> MetaResult {
	let path: PathBuf = path.into();
	if !path.exists() {
		return Err(MetaError::NotExist(path))
	}

	if path.is_dir() {
		return Err(MetaError::NotAFile(path));
	}

	let content = match fs::read(&path) {
		Ok(value) => value,
		Err(error) => return Err(MetaError::Io((path, error)))
	};
	let interpret: MetaFormat = match js::from_slice(&content) {
		Ok(value) => value,
		Err(error) => return Err(MetaError::Serde((path, error)))
	};

	if interpret.compiler_options.is_none() {
		return Err(MetaError::NoCompilerOptions(path));
	}

	Ok(())
}

use std::io;
/// General error type for `check_meta()` function
#[derive(Debug)]
pub enum MetaError {
	/// Emit when `path` does not exists
	NotExist(PathBuf),
	/// Emit when `path` is directory
	NotAFile(PathBuf),
	/// Emit when `serde_json` cannot parse JSON
	Serde((PathBuf, serde_json::Error)),
	/// Emit when I/O error occur
	Io((PathBuf, io::Error)),
	/// Emit when `compiler_options` is not found inside `path`
	NoCompilerOptions(PathBuf)
}

use colored::*;
use std::fmt;
impl fmt::Display for MetaError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			MetaError::NotAFile(path) => write!(f, "'{}' is directory.", path.display().to_string().cyan()),
			MetaError::NotExist(path) => write!(f, "'{}' does not exists.", path.display().to_string().cyan()),
			MetaError::Serde((path, error)) => write!(f, "[{}] {}", path.display().to_string().green(), error),
			MetaError::Io((path, error)) => write!(f, "[{}] {}", path.display().to_string().green(), error),
			MetaError::NoCompilerOptions(path) => write!(f, "'{}' does not have {} field.", path.display().to_string().cyan(), "compiler_options".white().on_blue()),
		}
	}
}

/// Check if `path` has the correct extension for loot table script
pub fn is_loot_table_script(path: impl Into<PathBuf>) -> bool {
	let path: PathBuf = path.into();

	if let Some(extension) = path.extension() {
		if let Some(extension) = extension.to_str() {
			match extension {
				"ult" | "json.merge" | "megu" => return true,
				_ => false
			};
		};
	}

	false
}