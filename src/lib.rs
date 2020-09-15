use std::io;
use std::fs;
use std::path;
use std::os::windows::prelude::*;

pub use structures::Config;
pub use structures::Style;

mod structures;
mod term_helper;
mod tools;



pub fn run(config:structures::Config, style:structures::Style) -> Result<(), &'static str> {
	let short_config = structures::ShortConfig {
		path_bfr:config.path_bfr,
		level:config.level,
		max_level:config.max_level,
	};

	if config.repr == "tree" {
		let mut entries:Vec<structures::TreeEntry> = Vec::new();		
		if let Err(_e) = get_tree(short_config, &mut entries) {
			//if error
		}
		print_tree(&entries, &style);
	} else if config.repr == "grid" {
		let mut entries:Vec<structures::GridEntry> = Vec::new();		
		if let Err(_e) = get_grid(short_config, &mut entries) {
			//if error
		}
		print_grid(&entries, &style);
	}

	Ok(())
}

fn get_grid(config:structures::ShortConfig, vector:&mut Vec<structures::GridEntry>) -> Result<(), &'static str> {
	match config.path_bfr.read_dir() {
		Ok(dir) => {
			for entry in dir {
				if let Ok(entry) = entry {
					vector.push(get_grid_entry(&entry.path()));
				}
			}
		},
		Err(e) => {

		}
	}
	Ok(())
}

fn get_grid_entry(path:&path::PathBuf) -> structures::GridEntry {
	let mut size = String::new();
	let mut last_modified = String::new();
	let name = tools::get_path_name(&path);
	let mut entry_type = String::new();

	//windows platform metadata
	match fs::metadata(&path) {
		Ok(metadata) => {
			if path.is_dir() {
				entry_type = String::from("DIR");
				size = format!("-");
			} else if path.is_file() {
				entry_type = String::from("FILE");
				size = tools::round_size(metadata.file_size());
			} else {
				unimplemented!();
				//symlink
			}

			let seconds = metadata.last_write_time()/10000000;
			last_modified = seconds_to_date(seconds);
		},
		Err(_e) => {
			size = format!("-");
			last_modified = format!("-");
		}
	}

	return structures::GridEntry {
		size,
		name,
		entry_type,
		last_modified,
	};
}

fn print_grid(vector:&Vec<structures::GridEntry>, style:&structures::Style) {
	let mut size_max = vector[0].size.len();
	let mut last_modified_max = 13;
	let mut name_max =  vector[0].name.len();

	for entry in vector {
		if entry.size.len() > size_max {
			size_max = entry.size.len();
		}
		if entry.name.len() > name_max {
			name_max = entry.name.len();
		}
	}

	//11="Last Modified"-2 and the same for the rest
	println!("Last Modified{}Size{}Name", " ".repeat(last_modified_max-11),
							  " ".repeat(size_max-2));
	println!("{}  {}  {}",    "-".repeat(last_modified_max),
					"-".repeat(size_max),
					"-".repeat(name_max) );

	for entry in vector {
		term_helper::print_clr(format!("{:>1$}{2}", 
							entry.last_modified, 
							last_modified_max, 
							" ".repeat(2)), 
					style.last_modified_clr);

		term_helper::print_clr(format!("{:>1$}{2}",
							entry.size, 
							size_max,
							" ".repeat(2)),
					style.size_clr);

		if entry.entry_type == "DIR" {
			term_helper::println_clr(format!("{}", entry.name), 
					style.dir_clr);
		} else if entry.entry_type == "FILE" {
			term_helper::println_clr(format!("{}", entry.name), 
					style.file_clr);
		} else {
			unimplemented!();
		}
	}

}

fn get_tree(config:structures::ShortConfig, vector:&mut Vec<structures::TreeEntry>) -> Result<(), &'static str> {
	if config.level == 0 {
		println!("Listing in: {:?}", config.path_bfr.display());
		println!("");
	}

	if config.level > config.max_level {
		return Ok(());
	}

	match config.path_bfr.read_dir() {
		Ok(dir) => {
			for entry in dir {
				if let Ok(entry) = entry {
					let path = entry.path();
					vector.push(get_tree_entry(&path, config.level));
					if path.is_dir() {
						let new_config = structures::ShortConfig {
							path_bfr:path,
							level:config.level+1,
							..config
						};
						get_tree(new_config, vector);
					}
				}
			}
		}
		Err(e) => {
			let mut msg = "error";
			if e.kind() == io::ErrorKind::NotFound {
				msg = "File not found";
			} else if e.kind() == io::ErrorKind::PermissionDenied {
				msg = "Access denied";
			} else if e.kind() == io::ErrorKind::Other {
				msg = "Error while reading dir";
			}

			term_helper::print_clr(msg, term::color::BRIGHT_RED);
			return Err(msg);
		}
	}

	Ok(())
}

fn seconds_to_date(seconds:u64) -> String {
	let total_days = (seconds as f64)/86400.0;
	let years = seconds/31556926;
	let months = ((total_days - years as f64 * 365.24)/30.44) as u64;
	let days = total_days - (years as f64 * 365.24) - months as f64 * 30.44;
	format!("{} {} {}",days as u64, index_to_month(months+1), years+1601)
}

fn index_to_month(index:u64) -> String {
	match index {
		1 => String::from("Jan"),
		2 => String::from("Fev"),
		3 => String::from("Mar"),
		4 => String::from("Apr"),
		5 => String::from("May"),
		6 => String::from("Jau"),
		7 => String::from("Jui"),
		8 => String::from("Aou"),
		9 => String::from("Sep"),
		10 => String::from("Oct"),
		11 => String::from("Nov"),
		12 => String::from("Dec"),
		_ => String::from("-"),
	}
}

fn get_tree_entry(path:&path::PathBuf, level:u32) -> structures::TreeEntry {
	let mut size = String::new();
	let mut last_modified = String::new();
	let name = tools::get_path_name(&path);
	let mut entry_type = String::new();

	//windows platform metadata
	match fs::metadata(&path) {
		Ok(metadata) => {
			if path.is_dir() {
				entry_type = String::from("DIR");
				size = format!("-");
			} else if path.is_file() {
				entry_type = String::from("FILE");
				size = tools::round_size(metadata.file_size());
			} else {
				unimplemented!();
				//symlink
			}

			let seconds = metadata.last_write_time()/10000000;
			last_modified = seconds_to_date(seconds);
		},
		Err(_e) => {
			size = format!("-");
			last_modified = format!("-");
		}
	}

	return structures::TreeEntry {
		size,
		level,
		name,
		entry_type,
		last_modified,
	};
}

fn print_tree(vector:&Vec<structures::TreeEntry>, style:&structures::Style) {
	let mut size_max = vector[0].size.len();
	let mut last_modified_max=  vector[0].last_modified.len();
	for entry in vector {
		if entry.size.len() > size_max {
			size_max = entry.size.len();
		}
		if entry.last_modified.len() > last_modified_max {
			last_modified_max = entry.last_modified.len();
		}
	}
	
	for entry in vector {
		term_helper::print_clr(format!("{:>1$}", entry.last_modified, last_modified_max), style.size_clr);
		term_helper::print_clr(format!("  "), style.size_clr);
		term_helper::print_clr(format!("{:>1$}", entry.size, size_max), style.size_clr);
		print!("{}|-> ", "|     ".repeat(entry.level as usize));
		if entry.entry_type == "DIR" {
			term_helper::println_clr(&entry.name, style.dir_clr);
		} else if entry.entry_type == "FILE" {
			term_helper::println_clr(&entry.name, style.file_clr);
		} else {
			unimplemented!();
		}
	}
}