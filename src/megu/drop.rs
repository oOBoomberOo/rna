/// Possible type within `type` field of MeguDrop.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropType {
	Item,
	Tag,
	LootTable,
	Group,
	Alternatives,
	Sequence,
	Dynamic,
	Empty
}

impl Default for DropType {
	fn default() -> DropType {
		DropType::Item
	}
}

impl DropType {
	/// Get DropType from Namespace.
	/// 
	/// This method can fail if DropType is not valid.
	pub fn get_drop_type(value: impl Into<String>) -> Result<DropType, DropTypeError> {
		let value = value.into();
		let namespace = Namespace::decode(&value)?;

		let kind = match namespace.prefix.as_ref() {
			"minecraft" => match namespace.suffix.as_ref() {
				"item" => DropType::Item,
				"tag" => DropType::Tag,
				"loot_table" => DropType::LootTable,
				"group" => DropType::Group,
				"alternatives" => DropType::Alternatives,
				"sequence" => DropType::Sequence,
				"dynamic" => DropType::Dynamic,
				"empty" => DropType::Empty,
				_ => return Err(DropTypeError::InvalidType(value))
			},
			_ => return Err(DropTypeError::InvalidType(value))
		};

		Ok(kind)
	}
}

use serde_json::Value;
/// A data structure representing each item in the `pools` field.
#[derive(Debug, Clone, PartialEq)]
pub struct MeguDrop {
	r#unsafe: bool,
	kind: DropType,
	name: Option<String>,
	children: Option<Vec<MeguDrop>>,
	conditions: Vec<Value>,
	functions: Vec<Value>
}

use super::{Namespace, DecodeError};
impl MeguDrop {
	pub(crate) fn new(kind: impl Into<DropType>, name: impl Into<Option<String>>, children: impl Into<Option<Vec<MeguDrop>>>, conditions: impl Into<Vec<Value>>, functions: impl Into<Vec<Value>>, r#unsafe: bool) -> MeguDrop {
		let kind = kind.into();
		let name = name.into();
		let children = children.into();
		let conditions = conditions.into();
		let functions = functions.into();
		
		MeguDrop { kind, name, children, conditions, functions, r#unsafe }
	}

	/// Check for "unsafe" type which is usually a type that can create nested structure inside loot table.
	fn is_unsafe(kind: DropType) -> bool {
		match kind {
			DropType::Alternatives |
			DropType::Group |
			DropType::Sequence
			=> true,
			_ => false
		}
	}

	/// Create MeguDrop from a drop template for `serde_json` to interpret
	/// 
	/// This method can fail if DropType is not valid.
	pub fn from_drop_format(format: DropFormat) -> Result<MeguDrop, DropTypeError> {
		let kind = DropType::get_drop_type(&format.r#type)?;
		let name = format.name;
		let children = MeguDrop::get_children(format.children)?;
		let conditions = format.conditions.unwrap_or_default();
		let functions = format.functions.unwrap_or_default();

		let r#unsafe = format.r#unsafe.unwrap_or_default();

		if r#unsafe != MeguDrop::is_unsafe(kind) {
			return Err(DropTypeError::NotAllow(format.r#type));
		}

		let result = MeguDrop::new(kind, name, children, conditions, functions, r#unsafe);
		
		Ok(result)
	}

	/// Safely convert `DropFormat` to `MeguDrop`
	fn get_children(format: Option<Vec<DropFormat>>) -> Result<Option<Vec<MeguDrop>>, DropTypeError> {
		match format {
			None => Ok(None),
			Some(childs) => {
				let result: Result<Vec<_>, _> = childs.into_iter().map(MeguDrop::from_drop_format).collect();

				// This is probably the stupidiest syntax I have ever seen.
				// TODO: Fix this
				Ok(Some(result?))
			}
		}
	}
}

impl From<DropFormat> for MeguDrop {
	fn from(format: DropFormat) -> MeguDrop {
		MeguDrop::from_drop_format(format).unwrap()
	}
}

/// General error type for `DropType`
#[derive(Debug, PartialEq)]
pub enum DropTypeError {
	/// Emit when it cannot decode Namespace
	DecodeError(DecodeError),
	/// Emit when you're trying to use unsafe type in a Drop without `"unsafe": true` keyword
	NotAllow(String),
	/// Emit when `type` field is not a valid type
	InvalidType(String)
}

use colored::*;
use std::fmt;
impl From<DecodeError> for DropTypeError {
	fn from(error: DecodeError) -> DropTypeError {
		DropTypeError::DecodeError(error)
	}
}
impl fmt::Display for DropTypeError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			DropTypeError::DecodeError(error) => write!(f, "{}", error),
			DropTypeError::InvalidType(original) => write!(f, "'{}' is not a valid type name.", original.cyan()),
			DropTypeError::NotAllow(kind) => write!(f, "'{}' is {} without {} keyword.", kind.cyan(), "not allow".red(), "unsafe".white().on_red()),
		}
	}
}

use serde::{Serialize, Deserialize};
/// A template structure for `serde_json` to use.
/// 
/// `name` and `children` should never be `Some()` at the same time.
#[derive(Serialize, Deserialize)]
pub struct DropFormat {
	pub r#unsafe: Option<bool>,
	pub r#type: String,
	pub name: Option<String>,
	pub children: Option<Vec<DropFormat>>,
	pub functions: Option<Vec<Value>>,
	pub conditions: Option<Vec<Value>>
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn get_drop_type_from_string() {
		assert_eq!(DropType::get_drop_type("minecraft:item"), Ok(DropType::Item));
	}

	#[test]
	fn get_drop_type_from_string_without_prefix() {
		assert_eq!(DropType::get_drop_type("item"), Ok(DropType::Item));
	}

	#[test]
	fn is_drop_type_unsafe() {
		assert!(MeguDrop::is_unsafe(DropType::Alternatives));
	}
}