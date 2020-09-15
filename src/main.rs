use std::env;
use iron::Config;
use iron::Style;
use term;


fn main() {
	let config = Config::new(env::args()).unwrap_or_else(|e| {
		eprintln!("Error parsing arguments: {}",e);
		std::process::exit(1);
	});

	let style = Style {
		dir_clr:term::color::BRIGHT_GREEN,
		file_clr:term::color::BRIGHT_WHITE,
		size_clr:term::color::BRIGHT_CYAN,
		last_modified_clr:term::color::BRIGHT_MAGENTA,
	};

	if let Err(e) = iron::run(config, style) {
		eprintln!("Application error: {}",e);
		std::process::exit(1);
	}
}

// cd Documents\rust\iron