use chrono::Local;
use colored::*;
use dotenv::dotenv;

pub mod utils {
	#[macro_export]
	macro_rules! remove_quotes {
		($s:expr) => {{
			$s.replace(&['\"', '\''][..], "")
		}};
	}

	#[macro_export]
	macro_rules! clear_console {
		() => {{
			print!("\x1B[2J\x1B[1;1H");
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
			static ref RE_IMPORT: Regex =
				Regex::new(&*dotenv::var("IMPORTER_REGEX").unwrap_or(r#"importar\((.*?)\)|import\((.*?)\)"#.to_string())).unwrap();
			static ref RE_COMMENTS: Regex = Regex::new(
				&*dotenv::var("COMMENT_REGEX").unwrap_or(r#"(/\*([^*]|[\r\n]|(\*+([^*/]|[\r\n])))*\*+\/)|(\/\/.*)|(#.*)"#.to_string())
			)
			.unwrap();
		};
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

	pub fn find_tabs(line: &str) -> &str {
		lazy_static! {
			static ref RE_TABS: Regex = Regex::new(&*dotenv::var("TABS_REGEX").unwrap_or(r#"^(?:( )+|\t+)"#.to_string())).unwrap();
		}
		match RE_TABS.captures(line) {
			Some(caps) => return caps.get(0).unwrap().as_str(),
			None => return ""
		};
	}

	pub fn find_commented(line: &str) -> bool {
		lazy_static! {
			static ref RE_TABS: Regex = Regex::new(
				&*dotenv::var("COMMENT_REGEX").unwrap_or(r#"(/\*([^*]|[\r\n]|(\*+([^*/]|[\r\n])))*\*+/)|(//.*)|(#.*)"#.to_string())
			)
			.unwrap();
		}
		match RE_TABS.captures(line) {
			Some(caps) => return caps.get(0).unwrap().as_str().len() > 0,
			None => return false
		};
	}

	#[test]
	fn _find_tabs_test() {
		assert_eq!(self::find_tabs("teste"), "");
		assert_eq!(self::find_tabs("teste\t\t"), "");
		assert_eq!(self::find_tabs("\t\t\tteste"), "\t\t\t");
		assert_eq!(self::find_tabs("   teste"), "   ");
	}

	#[test]
	fn _find_import_test() {
		assert!(self::find_import("importar(\"./somepath/file61_test.cs\")").is_some());
		assert!(self::find_import("import(\"file52_test.cs\")").is_some());
		assert_eq!(self::find_import("importar(\"./somepath/file21_test.cs\")").unwrap(), "\"./somepath/file21_test.cs\"");
		assert_eq!(self::find_import("import(\"file32_test.cs\")").unwrap(), "\"file32_test.cs\"");
	}

	#[test]
	fn _find_commented_test() {
		assert!(self::find_commented("//teste"));
		assert!(self::find_commented("/*tester 2 kkkkk*/"));
		assert!(self::find_commented("#teste/;3"));
		assert!(!self::find_commented("a b c d e / f g h i * k / 3"));
		assert!(!self::find_commented("/*/"));
	}
}

mod includer {
	use colored::*;
	use std::{
		fs::*,
		io::{self, BufRead},
		path::Path
	};
	pub fn open_output() -> File {
		let _path_ss = &*dotenv::var("OUTPUT_FILE").unwrap_or("./release/transpiled.cs".to_string());
		let _path_tc = Path::new(_path_ss);
		let mut output_file_unw = OpenOptions::new().write(true).truncate(true).open(_path_tc);

		if output_file_unw.is_err() {
			println!("{}", "Creating output file...".red());

			create_dir_all(_path_tc.parent().unwrap()).unwrap();
			File::create(_path_tc).unwrap();
			output_file_unw = std::fs::OpenOptions::new().write(true).truncate(true).open(_path_tc);
			println!("{}", "File created!".red());
		}
		output_file_unw.unwrap()
	}

	fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
	where P: AsRef<Path> {
		let file = File::open(filename)?;
		Ok(io::BufReader::new(file).lines())
	}

	pub fn import(path: &str, output_file: &mut File, mut idents: Option<&str>, after_import_env: bool) -> Result<(), ()> {
		if after_import_env {
			io::Write::write(output_file, &*dotenv::var("AFTER_IMPORT").unwrap_or("".to_string()).as_bytes()).unwrap();
		}
		if idents.is_none() {
			idents = Some("");
		}
		if let Ok(lines) =
			self::read_lines(crate::remove_quotes!(format!("{}{}", &*dotenv::var("WATCH_FOLDER").unwrap_or("./src/".to_string()), path)))
		{
			for line in lines {
				if let Ok(_content) = line {
					let line_content = format!("{tabs}{content}\n", tabs = idents.unwrap(), content = &_content);
					if let Some(_path) = crate::finder::find_import(&_content) {
						if !crate::finder::find_commented(&_content) {
							println!("{}", format!("importing {}...", crate::remove_quotes!(_path)).green());
							self::import(_path, output_file, Some(crate::finder::find_tabs(&_content)), true).unwrap_or(());
							continue;
						}
					}
					io::Write::write(output_file, line_content.as_bytes()).unwrap();
				}
			}
		} else {
			println!("{}", format!("File path '{}' could not be opened", path).bright_red());
			return Err(());
		}
		Ok(())
	}

	#[test]
	fn _import() {
		let mut output_file = self::open_output();
		assert!(self::import("main.rs", &mut output_file, None, false).is_ok());
		assert!(self::import("./notexistfile.txt", &mut output_file, None, false).is_err());
	}
}

mod watcher {
	use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
	use std::{sync::mpsc::channel, time::Duration};

	pub fn main(callback: fn() -> ()) {
		callback();

		let (tx, rx) = channel();
		let mut watcher = watcher(tx, Duration::from_millis(500)).unwrap();
		watcher.watch(&*dotenv::var("WATCH_FOLDER").unwrap_or("./src".to_string()), RecursiveMode::Recursive).unwrap();

		loop {
			match rx.recv() {
				Ok(event) => match event {
					DebouncedEvent::Write(_) => callback(),
					_ => ()
				},
				Err(e) => println!("watch error: {:?}", e)
			}
		}
	}
}

fn main() {
	dotenv().ok();

	fn callback() -> () {
		clear_console!();
		println!("{}", "Retranspiling...".cyan().bold());
		let mut output_file = includer::open_output();
		includer::import(&*dotenv::var("INPUT_FILE").unwrap_or("main.cs".to_string()), &mut output_file, None, false).unwrap();
		println!("{}", format!("Last transpile at {}", Local::now().format("%H:%M:%S")).purple().bold());
	}

	watcher::main(callback);
}
