mod megu;

pub use megu::{MeguScript, MeguDrop, Namespace};

fn main() {
	if let Err(result) = megu::interpret_file("test/min.megu") {
		println!("{}", result);
	}
}