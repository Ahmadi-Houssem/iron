use std::env;
use std::path::PathBuf;
use term;


#[derive(Debug)]
pub struct Config {
	pub path_bfr:PathBuf,
	pub level:u32,
	pub max_level:u32,
	pub repr:String,
}

#[derive(Debug)]
pub struct ShortConfig {
	pub path_bfr:PathBuf,
	pub level:u32,
	pub max_level:u32,
}

#[derive(Debug)]
pub struct TreeEntry {
	pub size:String,
	pub level:u32,
	pub name:String,
	pub entry_type:String,
	pub last_modified:String,
}

#[derive(Debug)]
pub struct GridEntry {
	pub size:String,
	pub name:String,
	pub entry_type:String,
	pub last_modified:String,
}

pub struct Style {
	pub file_clr:term::color::Color,
	pub dir_clr:term::color::Color,
	pub size_clr:term::color::Color,
	pub last_modified_clr:term::color::Color,
}

impl Config {
	pub fn new(mut args:env::Args) -> Result<Config, &'static str> {
		let mut path_bfr = PathBuf::new();
		let mut is_path_set = false;
		let mut max_level:u32 = 1;
		let mut repr = String::from("tree");

		args.next();

		for argument in args {
			if argument.contains("dir") {
				let tmp = Config::get_arg_value(&argument);
				path_bfr = PathBuf::from(tmp);
				if !path_bfr.exists() {
					return Err("specified dir not found!")
				}
				is_path_set = true;
			}
			if argument.contains("--max_level") {
				max_level = Config::get_arg_value(&argument)
					.trim()
					.parse()
					.unwrap_or_else(|_e| 1);
			}
			if argument.contains("grid") { //replace with --grid
				repr = String::from("grid");
			}
		}


		if !is_path_set {
			if let Ok(bfr) = std::env::current_dir() {
				path_bfr = bfr;
			} else {
				return Err("could not get current dir");
			}
		}

		Ok(Config {
			path_bfr,
			level:0,
			max_level,
			repr,
		})
	}

	fn get_arg_value(s:&String) -> String {
		match s.find("=") {
			Some(i) => String::from(&s[i+1..]),
			None => String::new(),
		}
	}
}