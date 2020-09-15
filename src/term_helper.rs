use term;
use std::io::{self, Write};

pub fn print_clr<T:std::fmt::Display>(msg:T, clr:term::color::Color) {
	let mut terminal = term::stdout().unwrap();
	terminal.fg(clr).unwrap();
	terminal.attr(term::Attr::Bold).unwrap();
	print!("{}", msg);
	io::stdout().flush().unwrap();
	terminal.reset().unwrap();
}

pub fn println_clr<T:std::fmt::Display>(msg:T, clr:term::color::Color) {
	let mut terminal = term::stdout().unwrap();
	terminal.fg(clr).unwrap();
	terminal.attr(term::Attr::Bold).unwrap();
	println!("{}", msg);
	terminal.reset().unwrap();
}