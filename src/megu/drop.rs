#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropType {
	Item,
	Alternative
}

impl Default for DropType {
	fn default() -> DropType {
		DropType::Item
	}
}

use serde_json::Value;
#[derive(Debug, Clone, PartialEq)]
pub struct MeguDrop {
	r#unsafe: bool,
	kind: DropType,
	name: String,
	conditions: Vec<Value>,
	functions: Vec<Value>
}

use super::{Namespace, DecodeError};
impl MeguDrop {
	pub fn new(kind: impl Into<DropType>, name: impl Into<String>, conditions: impl Into<Vec<Value>>, functions: impl Into<Vec<Value>>, r#unsafe: bool) -> MeguDrop {
		let kind = kind.into();
		let name = name.into();
		let conditions = conditions.into();
		let functions = functions.into();
		
		MeguDrop { kind, name, conditions, functions, r#unsafe }
	}

	fn is_unsafe(kind: DropType) -> bool {
		kind == DropType::Alternative
	}

	pub fn get_drop_type(value: impl Into<String>) -> Result<DropType, DropTypeError> {
		let value = value.into();
		let namespace = Namespace::decode(&value)?;

		let kind = match namespace.prefix.as_ref() {
			"minecraft" => match namespace.suffix.as_ref() {
				"item" => DropType::Item,
				_ => return Err(DropTypeError::InvalidType(value))
			},
			_ => return Err(DropTypeError::InvalidType(value))
		};

		Ok(kind)
	}

	pub fn from_drop_format(format: DropFormat) -> Result<MeguDrop, DropTypeError> {
		let kind = MeguDrop::get_drop_type(format.r#type)?;
		let name = format.name;
		let conditions = format.condiions.unwrap_or_default();
		let functions = format.functions.unwrap_or_default();

		let r#unsafe = match format.r#unsafe {
			Some(value) => value,
			None => MeguDrop::is_unsafe(kind)
		};

		let result = MeguDrop {
			r#unsafe,
			kind,
			name,
			conditions,
			functions
		};
		
		Ok(result)
	}
}

impl From<DropFormat> for MeguDrop {
	fn from(format: DropFormat) -> MeguDrop {
		MeguDrop::from_drop_format(format).unwrap()
	}
}

#[derive(Debug, PartialEq)]
pub enum DropTypeError {
	DecodeError(DecodeError),
	InvalidType(String)
}

impl From<DecodeError> for DropTypeError {
	fn from(error: DecodeError) -> DropTypeError {
		DropTypeError::DecodeError(error)
	}
}

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct DropFormat {
	r#unsafe: Option<bool>,
	r#type: String,
	name: String,
	functions: Option<Vec<Value>>,
	condiions: Option<Vec<Value>>
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn get_drop_type_from_string() {
		assert_eq!(MeguDrop::get_drop_type("minecraft:item"), Ok(DropType::Item));
	}

	#[test]
	fn get_drop_type_from_string_without_prefix() {
		assert_eq!(MeguDrop::get_drop_type("item"), Ok(DropType::Item));
	}

	#[test]
	fn is_drop_type_unsafe() {
		assert!(MeguDrop::is_unsafe(DropType::Alternative));
	}
}