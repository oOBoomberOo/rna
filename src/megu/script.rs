use super::{Extension, Namespace, DecodeError, MeguDrop, DropFormat, ExtensionError};
use super::drop;
use std::error;
use std::collections::HashMap;

/// MeguScript is a data structure for loot table script
#[derive(Debug, PartialEq, Clone, Default)]
pub struct MeguScript {
	kind: Option<String>,
	extend: Option<Extension>,
	pools: HashMap<Namespace, MeguDrop>,
	remove: Vec<Namespace>
}

type PoolFormat = Result<HashMap<Namespace, MeguDrop>, drop::DropTypeError>;

impl MeguScript {
	/// Create new MeguScript
	fn new(kind: impl Into<Option<String>>, extend: impl Into<Option<Extension>>, pools: impl Into<HashMap<Namespace, MeguDrop>>, remove: impl Into<Vec<Namespace>>) -> MeguScript {
		let kind = kind.into();
		let extend = extend.into();
		let pools = pools.into();
		let remove = remove.into();
		MeguScript { kind, extend, pools, remove }
	}

	/// Convert JSON Template of loot table's pools into `MeguDrop`
	fn from_pools_format(format: HashMap<String, DropFormat>) -> PoolFormat {
		let mut result = HashMap::default();

		format
			.into_iter()
			.map(|(key, value)| (key, MeguDrop::from_drop_format(value)))
			.try_for_each(|(key, value)| -> Result<(), drop::DropTypeError> {
				let key = Namespace::decode(key)?;
				let value = value?;

				result.insert(key, value);
				Ok(())
			})?;

		Ok(result)
	}

	/// Create new MeguScript from `ScriptFormat` which is a template structure for `serde_json`
	fn from_script_format(format: ScriptFormat) -> Result<MeguScript, ScriptFormatError> {
		let kind = format.kind;
		let extend = match format.extend {
			Some(value) => Some(Extension::get_extension(value)?),
			None => None
		};

		let pools = MeguScript::from_pools_format(format.pools)?;
		let remove: Result<Vec<Namespace>, DecodeError> = match format.remove {
			Some(value) => value.into_iter().map(Namespace::decode).collect(),
			None => Ok(Vec::default())
		};
		let remove = remove?;

		let result = MeguScript::new(kind, extend, pools, remove);

		Ok(result)
	}

	/// Create new MeguScript from `path`
	/// 
	/// # Errors
	/// This method can fail when:
	/// - ScriptFormat emit error (i.e. Invalid Syntax)
	/// - I/O emit error (i.e. Cannot read file from path)
	/// - Serde emit Error (i.e. JSON Error)
	/// 
	/// # Example
	/// ```
	/// # use rna::MeguScript;
	/// let script = MeguScript::from_path("test/min.megu").unwrap();
	/// ```
	pub fn from_path(path: impl Into<PathBuf>) -> Result<MeguScript, ReadError> {
		let path = path.into();
		let content = fs::read(path)?;
		let format: ScriptFormat = js::from_slice(&content)?;
		let result = MeguScript::from_script_format(format)?;

		Ok(result)
	}

	/// Merge this script to `other` script.
	/// This method will mutate `other` but not `self`.
	pub fn merge(&self, other: &mut MeguScript) {
		for (key, value) in &self.pools {
			other.pools.insert(key.clone(), value.clone());
		}

		other.remove.append(&mut self.remove.clone());
	}

	/// Compile `Extension` inside `extend` (if not `None`).
	pub fn compile(&self) -> Result<MeguScript, ReadError> {
		let mut result: MeguScript = MeguScript::default();

		if let Some(extension) = &self.extend {
			let extension = extension.compile()?;
			result.kind = extension.kind;
			result.extend = extension.extend;
			result.pools = extension.pools;
			result.remove = extension.remove;
		}

		self.merge(&mut result);

		Ok(result)
	}

	/// Search through `pools` field and remove any `Drop` that's listed inside `remove` field.
	pub fn remove_drops(&mut self) -> Vec<Option<MeguDrop>> {
		self.remove
			.clone()
			.iter()
			.map(|namespace| self.pools.remove(&namespace))
			.collect()
	}
}

use std::path::PathBuf;
use std::fs;
use serde_json as js;
impl From<PathBuf> for MeguScript {
	fn from(path: PathBuf) -> MeguScript {
		MeguScript::from_path(path).unwrap()
	}
}
impl From<ScriptFormat> for MeguScript {
	fn from(format: ScriptFormat) -> MeguScript {
		MeguScript::from_script_format(format).unwrap()
	}
}

use serde::{Serialize, Deserialize};
/// Template structure for `serde_json` to use.
#[derive(Serialize, Deserialize)]
pub struct ScriptFormat {
	#[serde(rename = "type")]
	pub kind: Option<String>,
	pub extend: Option<String>,
	pub pools: HashMap<String, DropFormat>,
	pub remove: Option<Vec<String>>
}

/// General error type for decoding MeguScript
#[derive(Debug, PartialEq)]
pub enum ScriptFormatError {
	/// ExtensionError emit when there's something wrong in the `extend` field.
	Extension(ExtensionError),
	/// DropTypeError emit when there's something wrong in the `type` field in a `MeguDrop`
	DropType(drop::DropTypeError),
	/// DecodeError emit when there's an error in Namespace
	Decode(DecodeError)
}

impl From<ExtensionError> for ScriptFormatError {
	fn from(error: ExtensionError) -> ScriptFormatError {
		ScriptFormatError::Extension(error)
	}
}
impl From<drop::DropTypeError> for ScriptFormatError {
	fn from(error: drop::DropTypeError) -> ScriptFormatError {
		ScriptFormatError::DropType(error)
	}
}
impl From<DecodeError> for ScriptFormatError {
	fn from(error: DecodeError) -> ScriptFormatError {
		ScriptFormatError::Decode(error)
	}
}
impl fmt::Display for ScriptFormatError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ScriptFormatError::Extension(error) => write!(f, "{}", error),
			ScriptFormatError::DropType(error) => write!(f, "{}", error),
			ScriptFormatError::Decode(error) => write!(f, "{}", error),
		}
	}
}

use std::io;
/// General error type for `from_path()` function
#[derive(Debug)]
pub enum ReadError {
	/// Emit when there are syntax error in `ScriptFormat`
	ScriptFormat(ScriptFormatError),
	/// Emit when there are I/O error (i.e. cannot read file)
	Io(io::Error),
	/// Emit when there are JSON error (i.e. invalid JSON syntax)
	Serde(js::Error)
}

use std::fmt;
impl From<ScriptFormatError> for ReadError {
	fn from(error: ScriptFormatError) -> ReadError {
		ReadError::ScriptFormat(error)
	}
}
impl From<io::Error> for ReadError {
	fn from(error: io::Error) -> ReadError {
		ReadError::Io(error)
	}
}
impl From<js::Error> for ReadError {
	fn from(error: js::Error) -> ReadError {
		ReadError::Serde(error)
	}
}
impl fmt::Display for ReadError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ReadError::ScriptFormat(error) => write!(f, "{}", error),
			ReadError::Io(error) => write!(f, "{}", error),
			ReadError::Serde(error) => write!(f, "{}", error),
		}
	}
}
impl error::Error for ReadError {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn create_new_script() {
		assert_eq!(
			MeguScript::new(
				None,
				Some(Extension::new("minecraft/entities/creeper")),
				HashMap::default(),
				Vec::default()
			),
			MeguScript {
				kind: None,
				extend: Some(Extension::new("minecraft/entities/creeper")),
				pools: HashMap::default(),
				remove: Vec::default()
			}
		);
	}

	use crate::megu::drop::DropType;
	#[test]
	fn try_convert_pools() {
		let value = r#"
		{
			"type": "minecraft:item",
			"name": "minecraft:emerald"
		}
		"#;
		let value: DropFormat = js::from_str(value).unwrap();
		let mut map: HashMap<String, DropFormat> = HashMap::default();
		map.insert("test".to_string(), value);

		let value = MeguScript::from_pools_format(map).unwrap();

		let mut expect: HashMap<Namespace, MeguDrop> = HashMap::default();
		expect.insert(Namespace::new("minecraft", "test"), MeguDrop::new(
			DropType::Item,
			Some("minecraft:emerald".to_string()),
			None,
			Vec::default(),
			Vec::default(),
			false
		));

		assert_eq!(value, expect);
	}
}