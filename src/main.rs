mod megu;

pub use megu::{MeguScript, MeguDrop, Namespace};

fn main() {
	if let Err(error) = stuff() {
		eprintln!("{}", error);
	}
}

use std::fs;
use std::error::Error;
fn stuff() -> Result<(), Box<dyn Error>> {
	let scripts: Result<Vec<_>, _> = fs::read_dir("test")?
		.filter_map(|entry| entry.ok())
		.map(|entry| megu::interpret_file(entry.path()))
		.collect();
	let result = megu::merge(&scripts?)?;

	println!("{:#?}", result);

	Ok(())
}