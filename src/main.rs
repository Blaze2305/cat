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
						.version("0.2.0")
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
						.arg(Arg::with_name("non_empty_nums")
								.short("b")
								.long("number-nonblank")
								.takes_value(false)
								.help("number non empty output lines, overrides -n")
						)
						.arg(Arg::with_name("show_ends")
								.short("E")
								.long("show-ends")
								.takes_value(false)
								.help("Show $ at the end of each line")
						)
						.arg(Arg::with_name("squeeze_blank")
								.short("s")
								.long("squeeze-blank")
								.takes_value(false)
								.help("suppress repeated empty output lines")
						)
						.arg(Arg::with_name("show_tabs")
								.short("t")
								.long("show-tabs")
								.takes_value(false)
								.help("sdiaply TAB characters as ^I")
						)
					.get_matches();
					

	if matches.occurrences_of("files") > 0{
		let files :Vec<&str>= matches.values_of("files").unwrap().collect();


		let mut line_numbering = matches.is_present("line_nums");

		let non_empty_numbering = matches.is_present("non_empty_nums");
		if non_empty_numbering {
			line_numbering = false;
		}

		let show_ends = matches.is_present("show_ends");

		let squeeze_blanks = matches.is_present("squeeze_blank");

		let show_tabs = matches.is_present("show_tabs");

		for file_name in &files {
			match get_file_data(file_name,line_numbering,non_empty_numbering,show_ends,squeeze_blanks,show_tabs){
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


fn get_file_data(file_name : &str,line_num : bool,non_empty : bool,show_ends : bool,squeeze_blank : bool, show_tabs : bool) -> Result<String,io::Error> {
	let file = fs::read_to_string(file_name)?;
	let mut file_data :String = String::new();
	let mut count : u32 = 1;
	let mut repeated_blank : bool;
	let num_width = file.matches("\n").count().to_string().len() + 1;
	if line_num || non_empty || squeeze_blank{
		for line in file.split("\n"){
			let formatted : String;
			if line_num || (non_empty && !line.replace("\r","").is_empty()){
				formatted = format!("{:0width$}",count,width = num_width);
				count+=1;
				repeated_blank = false;
			}else{
				formatted = format!("{:0width$}"," ",width = num_width);
				repeated_blank = true;
			}

			if squeeze_blank && repeated_blank{
				continue;
			}

			file_data += &format_output_string(line,formatted, line_num || non_empty);
		}
	}else{
		file_data = file.to_string();
	}

	if show_ends {
		file_data = file_data.replace("\n", "$\n");
	}

	if show_tabs {
		file_data = file_data.replace("\t","^I");
	}

	Ok(file_data)
}

fn format_output_string(line : &str, line_number : String, show_num : bool) -> String {
	if show_num {
		line_number + "|   " + line + "\n"
	}else{
		line.to_string() + &"\n"
	}
}