use dotenv::dotenv;

pub mod utils {
	#[macro_export]
	macro_rules! remove_quotes {
		($s:expr) => {{
			$s.replace(&['\"', '\''][..], "")
		}};
	}

	#[test]
	fn _remove_quotes() {
		assert_eq!(crate::remove_quotes!("\"./file76_test.cs\""), "./file76_test.cs");
		assert_eq!(crate::remove_quotes!("'./file80_test.cs'"), "./file80_test.cs");
		assert_ne!(crate::remove_quotes!("'/to_to_/file23_test.cs'"), "'/to_to_/file23_test.cs'");
		assert_ne!(crate::remove_quotes!("\"/to_to_/file23_test.cs\""), "\"/to_to_/file23_test.cs\"");
	}
}

mod finder {
	use lazy_static::lazy_static;
	use regex::Regex;

	pub fn find_import(line: &str) -> Option<&str> {
		lazy_static! {
			static ref RE_IMPORT: Regex = Regex::new(
				&*dotenv::var("IMPORTER_REGEX").unwrap_or(r#"importar\((.*?)\)|import\((.*?)\)"#.to_string())
			)
			.unwrap();
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

	pub fn find_tabs(line: &str) -> Option<&str> {
		lazy_static! {
			static ref RE_TABS: Regex =
				Regex::new(&*dotenv::var("TABS_REGEX").unwrap_or(r#"^(?:( )+|\t+)"#.to_string())).unwrap();
		}
		match RE_TABS.captures(line) {
			Some(caps) => return Some(caps.get(0).unwrap().as_str()),
			None => return None
		};
	}

	#[test]
	fn _find_tabs() {
		assert_eq!(self::find_tabs("teste"), None);
		assert_eq!(self::find_tabs("teste\t\t"), None);
		assert_eq!(self::find_tabs("\t\t\tteste").unwrap(), "\t\t\t");
		assert_eq!(self::find_tabs("   teste").unwrap(), "   ");
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
		let _path_ss = &*dotenv::var("OUTPUT_FILE_unw").unwrap_or("./release/transpiled.cs".to_string());
		let _path_tc = Path::new(_path_ss);
		let mut output_file_unw = OpenOptions::new().write(true).truncate(true).open(_path_tc);

		if output_file_unw.is_err() {
			println!("Creating output file...");

			create_dir_all(_path_tc.parent().unwrap()).unwrap();
			File::create(_path_tc).unwrap();
			output_file_unw = std::fs::OpenOptions::new().write(true).truncate(true).open(_path_tc);
			println!("File created!");
		}
		let mut output_file = output_file_unw.unwrap();

		if let Ok(lines) = self::read_lines(crate::remove_quotes!(path)) {
			for line in lines {
				if let Ok(_content) = line {
					let line_content = format!("{}\n", &_content);
					if let Some(_path) = callcheck(_content.to_string()) {
						self::import(_path, callcheck).unwrap_or(());
					} else {
						io::Write::write(&mut output_file, line_content.as_bytes()).unwrap();
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

fn main() {
	dotenv().ok();
	//includer::import("./src/main.rs", |_content| None).unwrap();
}
