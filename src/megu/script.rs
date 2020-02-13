use super::{Extension, Namespace, DecodeError, MeguDrop, DropFormat, ExtensionError};
use super::drop;

use std::collections::HashMap;
#[derive(Debug, PartialEq, Clone)]
pub struct MeguScript {
	extend: Option<Extension>,
	pools: HashMap<Namespace, MeguDrop>,
	remove: Vec<Namespace>
}

impl MeguScript {
	fn new(extend: impl Into<Option<Extension>>, pools: impl Into<HashMap<Namespace, MeguDrop>>, remove: impl Into<Vec<Namespace>>) -> MeguScript {
		let extend = extend.into();
		let pools = pools.into();
		let remove = remove.into();
		MeguScript { extend, pools, remove }
	}

	fn from_pools_format(format: HashMap<String, DropFormat>) -> Result<HashMap<Namespace, MeguDrop>, drop::DropTypeError> {
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

	fn from_script_format(format: ScriptFormat) -> Result<MeguScript, ScriptFormatError> {
		let extend = match format.extend {
			Some(value) => Some(super::get_extension(value)?),
			None => None
		};

		let pools = MeguScript::from_pools_format(format.pools)?;
		let remove: Result<Vec<Namespace>, DecodeError> = format.remove.into_iter().map(Namespace::decode).collect();
		let remove = remove?;

		let result = MeguScript::new(extend, pools, remove);

		Ok(result)
	}
}

use std::path::PathBuf;
use std::fs;
use serde_json as js;
impl From<PathBuf> for MeguScript {
	fn from(path: PathBuf) -> MeguScript {
		let content = fs::read(path).unwrap();
		let format: ScriptFormat = js::from_slice(&content).unwrap();
		MeguScript::from(format)
	}
}

impl From<ScriptFormat> for MeguScript {
	fn from(format: ScriptFormat) -> MeguScript {
		MeguScript::from_script_format(format).unwrap()
	}
}

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct ScriptFormat {
	extend: Option<String>,
	pools: HashMap<String, DropFormat>,
	remove: Vec<String>
}

#[derive(Debug)]
enum ScriptFormatError {
	Extension(ExtensionError),
	DropType(drop::DropTypeError),
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn create_new_script() {
		assert_eq!(
			MeguScript::new(
				Some(Extension::Creeper),
				HashMap::default(),
				Vec::default()
			),
			MeguScript {
				extend: Some(Extension::Creeper),
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
			String::from("minecraft:emerald"),
			Vec::default(),
			Vec::default(),
			false
		));

		assert_eq!(value, expect);
	}
}