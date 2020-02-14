mod megu;

pub use megu::{MeguScript, MeguDrop, Namespace};

fn main() {
	match megu::interpret_file("test/min.megu") {
		Ok(script) => println!("{:#?}", script),
		Err(error) => eprintln!("{}", error)
	};
}