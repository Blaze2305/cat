use std::fs;
use std::io;
use clap::{Arg,App};

fn main() {
	ctrlc::set_handler(move || {
		print!("^C");
		std::process::exit(0);
	})
	.expect("Unable to set handler");
	
	let matches = App::new("cat")
						.version("0.1.0")
						.author("Blaze2305")
						.about("Unix style cat")
						.arg(Arg::with_name("line_nums")
								.short("n")
								.long("num")
								.takes_value(false)
								.help("print the line number for each file")
								
						)
						.arg(Arg::with_name("files")
								.value_name("FILES")
								.help("List of files to cat")
								.multiple(true)
								.number_of_values(1)
						)
					.get_matches();
					

	if matches.occurrences_of("files") > 0{
		let files :Vec<&str>= matches.values_of("files").unwrap().collect();

		for file_name in &files {
			match get_file_data(file_name,matches.is_present("line_nums")){
				Ok(file_data) => print!("--------------------\n{}\n--------------------\n{}\n--------------------\n",file_name,file_data),
				Err(err) => match err.kind(){
					io::ErrorKind::NotFound => println!("File Not Found {}",file_name),
					io::ErrorKind::PermissionDenied => println!("Permission Denied {}",file_name),
					_=> println!("Unkown Error"),
				},
			}
		}
	}else{
		let stdin = io::stdin();
		loop{
			let mut line = String::new();
			match stdin.read_line(&mut line) {
				Ok(_) => print!("{}",line),
				Err(_) => print!("Cannot read line")
			}
		}
	}

	
}


fn get_file_data(file_name : &str,line_num : bool) -> Result<String,io::Error> {
	let file = fs::read_to_string(file_name)?;
	let mut file_data :String = String::new();
	if line_num {
		let mut count : u32 = 1;
		let num_width = file.matches("\n").count().to_string().len();
		for line in file.split("\n"){
			let formatted = format!("{:0width$}",count,width = num_width);
			file_data += &(formatted + "|   " + line + "\n");
			count+=1;
		}
	}else{
		file_data = file.to_string();
	}
	Ok(file_data)
}
