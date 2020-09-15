pub fn round_size(size:u64) -> String {
	let mut unit = String::new();
	let mut divider = 1.0;

	if size < 1000 {
		unit = String::from("Bi");
		divider = 1.0;
	} else if size < 10000 {  //9,999
		unit = String::from("Ki");
		divider = 1000.0;
	} else if size < 100000 {  //99,999
		unit = String::from("Ki");
		divider = 1000.0;
	} else if size < 1000000 { //999,999
		unit = String::from("Ki");
		divider = 1000.0;
	} else if size < 10000000 {  
		unit = String::from("Mi");
		divider = 1000000.0;
	} else if size < 100000000 {
		unit = String::from("Mi");
		divider = 1000000.0;
	} else if size < 1000000000 {
		unit = String::from("Mi");
		divider = 1000000.0;
	} else if size < 10000000000 {
		unit = String::from("Gi");
		divider = 1000000000.0;
	} else if size < 100000000000 {
		unit = String::from("Gi");
		divider = 1000000000.0;
	} else if size < 1000000000000 {
		unit = String::from("Gi");
		divider = 1000000000.0;
	} else {
		unit = String::from("extra");
	}

	let t = ((size as f64)/divider).to_string();
	let t = match t.find('.') {
		Some(index) => {
			if index+3 <= t.len() {
				&t[..index+3]
			} else if index+2 <= t.len() {
				&t[..index+2]
			} else {
				&t[..index]
			}
			
		},
		None => {&t[..]}
	};

	format!("{}{}", t, unit)
}

pub fn get_path_name(path:&std::path::Path) -> String {
	path
		.components()
		.last()
		.unwrap()
		.as_os_str()
		.to_string_lossy()
		.to_string()
}