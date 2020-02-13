/// Namespace consist of 'prefix' and 'suffix'. 
/// where 'prefix' is any string before `:` and 'suffix' is any string after that
/// 
/// # Example Namespaces
/// - `minecraft:test` -> prefix: `minecraft`, suffix: `test`
/// - `megumin:explosion` -> prefix: `megumin`, suffix: `explosion`
/// - `boomber:hello_world` -> prefix: `boomber`, suffix: `hello_world`
/// 
/// # Examples
/// ## Creating Namespace
/// This method will create new namespace without any check for invalid character in namespace.
/// ```
/// # use rna::Namespace;
/// let namespace = Namespace::new("megumin", "explosion");
/// assert_eq!(namespace.prefix, "megumin");
/// assert_eq!(namespace.suffix, "explosion");
/// ```
/// 
/// 
/// ## Decode Namespace
/// This method will create new namespace while also check of any invalid syntax in namespace and will return `DecodeError` if that happened.
/// ```
/// # use rna::Namespace;
/// let namespace = Namespace::decode("megumin:explosion").unwrap();
/// assert_eq!(namespace.prefix, "megumin");
/// assert_eq!(namespace.suffix, "explosion");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Namespace {
	/// String that come before `:`
	pub prefix: String,
	/// String that come after `:`
	pub suffix: String
}

use regex::Regex;
impl Namespace {
	/// Manually create new Namespace
	pub fn new(prefix: impl Into<String>, suffix: impl Into<String>) -> Namespace {
		let prefix = prefix.into();
		let suffix = suffix.into();
		Namespace { prefix, suffix }
	}

	/// Create Namespace from a given string.  
	/// 
	/// # Errors
	/// This method can error when:
	/// - Input contain invalid characters for namespace
	/// - Input contain too many colons (`:`)
	/// - There's an error inside `regex` crate
	/// 
	/// # Examples
	/// ```
	/// # use rna::Namespace;
	/// assert_eq!(
	///    Namespace::decode("boomber:hello_world").unwrap(),
	///    Namespace::new("boomber", "hello_world")
	/// );
	/// ```
	/// 
	/// If no colon is provided, 'minecraft' prefix will be used.
	/// ```
	/// # use rna::Namespace;
	/// assert_eq!(
	///    Namespace::decode("without_prefix").unwrap(),
	///    Namespace::new("minecraft", "without_prefix")
	/// );
	/// ```
	pub fn decode(value: impl Into<String>) -> Result<Namespace, DecodeError> {
		let value = value.into();
		let namespace_validation = Regex::new(r#"^[a-z:\d/_-]+$"#)?;

		if !namespace_validation.is_match(&value) {
			return Err(DecodeError::InvalidNamespace(value));
		}

		let semicolon_counts = value.clone().chars().filter(|&c| c == ':').count();
		if semicolon_counts > 1 {
			return Err(DecodeError::TooManyColons(value));
		}

		let (prefix, suffix) = {
			if semicolon_counts == 1 {
				let result: Vec<&str> = value.split(':').take(2).collect();

				(result[0], result[1])
			}
			else {
				let result: &str = &value;
				("minecraft", result)
			}
		};

		let result = Namespace::new(prefix, suffix);
		Ok(result)
	}
}

/// Error handling for Namespace::decode() method
#[derive(Debug, PartialEq)]
pub enum DecodeError {
	/// Error cause from 'regex' crate
	RegexError(regex::Error),
	/// Cause when there is invalid character inside namespace.
	/// The original string is attached to this error.
	InvalidNamespace(String),
	/// Cause when there are too many colons inside namespace.
	/// The original string is attached to this error.
	TooManyColons(String)
}

/// Create Namespace from &str.  
/// This will `unwrap()` error emit from `Namespace::decode()` function.
impl From<&str> for Namespace {
	fn from(value: &str) -> Namespace {
		Namespace::decode(value).unwrap()
	}
}

impl From<regex::Error> for DecodeError {
	fn from(error: regex::Error) -> DecodeError {
		DecodeError::RegexError(error)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn decode_namespace_with_prefix() {
		assert_eq!(
			Namespace::from("boomber:test"),
			Namespace {
				prefix: String::from("boomber"),
				suffix: String::from("test")
			}
		);
	}

	#[test]
	#[should_panic]
	fn panic_on_invalid_namespace() {
		Namespace::from("this:namespace:IS invalid");
	}

	#[test]
	fn decode_namespace_without_prefix() {
		assert_eq!(
			Namespace::from("no_prefix"),
			Namespace {
				prefix: String::from("minecraft"),
				suffix: String::from("no_prefix")
			}
		);
	}

	#[test]
	#[should_panic]
	fn panic_on_invalid_namespace_without_prefix() {
		Namespace::from("This Namespace Is Not Valid");
	}
}