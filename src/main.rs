static DEFAULT_OUTPUT_FILE: &str = "./release/transpiled.cs";
static DEFAULT_INPUT_FILE: &str = "./src/main.cs";
static DEFAULT_IMPORTER_REGEX: &str = r#"importar\((.*?)\)|import\((.*?)\)"#;

pub mod utils {
	macro_rules! remove_quotes {
		($s:expr) => {{
			$s.replace(&['\"', '\''][..], "")
		}};
	}

	#[test]
	fn _remove_quotes() {
		assert_eq!(remove_quotes!("\"./file76_test.cs\""), "./file76_test.cs");
		assert_eq!(remove_quotes!("'./file80_test.cs'"), "./file80_test.cs");
		assert_ne!(remove_quotes!("'/to_to_/file23_test.cs'"), "'/to_to_/file23_test.cs'");
		assert_ne!(remove_quotes!("\"/to_to_/file23_test.cs\""), "\"/to_to_/file23_test.cs\"");
	}
}

mod finder {
	use lazy_static::lazy_static;
	use regex::Regex;

	pub fn find_import(line: &str) -> Option<&str> {
		lazy_static! {
			static ref RE_IMPORT: Regex = Regex::new(crate::DEFAULT_IMPORTER_REGEX).unwrap();
		}
		match RE_IMPORT.captures(line) {
			Some(caps) => {
				if let Some(caps) = caps.get(1) {
					return Some(caps.as_str());
				} else {
					return Some(caps.get(2).unwrap().as_str());
				}
			}
			None => return None
		};
	}

	#[test]
	fn _find_import() {
		assert!(self::find_import("importar(\"./somepath/file61_test.cs\")").is_some());
		assert!(self::find_import("import(\"file52_test.cs\")").is_some());
		assert_eq!(
			self::find_import("importar(\"./somepath/file21_test.cs\")").unwrap(),
			"\"./somepath/file21_test.cs\""
		);
		assert_eq!(self::find_import("import(\"file32_test.cs\")").unwrap(), "\"file32_test.cs\"");
	}
}

mod includer {
	use std::{
		fs::*,
		io::{self, BufRead},
		path::Path
	};

	fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
	where P: AsRef<Path> {
		let file = File::open(filename)?;
		Ok(io::BufReader::new(file).lines())
	}

	pub fn import(path: &str, callcheck: fn(String) -> Option<&'static str>) -> Result<(), ()> {
		//let output_file =
		// OpenOptions::new().read(true).create(true).write(true).truncate(true).open(crate::DEFAULT_OUTPUT_FILE); match
		// output_file { 	Err(why) => {
		//		println!("Output file cannot be opened! {}", why);
		//		return Err(());
		//	}
		//	Ok(_) => {
		//		println!("Output file opened...");
		//	}
		//};

		if let Ok(lines) = self::read_lines(path) {
			for line in lines {
				if let Ok(_content) = line {
					match callcheck(_content) {
						Some(_path) => self::import(_path, callcheck).unwrap_or(()), //Check path for more includes
						None => ()                                                   //Add line to output file
					}
				}
			}
		} else {
			println!("File path '{}' could not be opened", path);
			return Err(());
		}
		Ok(())
	}

	#[test]
	fn _import() {
		assert!(self::import("./src/main.rs", |_content| Some("a")).is_ok());
		assert!(self::import("./notexistfile.txt", |_content| None).is_err());
	}
}

fn main() {}
