use std::env;
use std::process;
use std::fs;
use std::io::{self, Write};
mod tokens;
mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args[0]);
    if args.len() > 2{
	println!("Usage: aleph [script]");
	process::exit(1);
    } else if args.len() == 2 {
	run_file(&args[0]);
    } else {
	run_prompt();
    }
    
    println!("Hello, world!");
}

fn run_file(file_path: &String){
    let code = fs::read_to_string(file_path)
	.expect("Something went wrong");

    match run(code){
	Ok(_) => {
	    println!("Execution complete!");
	}
	Err(error) => {
	    println!("Unexpected error in code");
	    process::exit(65);
	}
    }
}

fn run_prompt(){
    loop {
	print!("> ");
	io::stdout().flush();
	let mut input = String::new();
	match io::stdin().read_line(&mut input){
	    Ok(_n) => {
		run(input);
	    }
	    Err(error) => {
		println!("Invalid expression. ");
		println!("{}", error);
		continue;
	    }
	}
    }
}

fn run(code: String) -> Result<(), ()> {
    let scanner = scanner::Scanner::new(code);
    Ok(println!("Code: {}", code))
}

fn error(line: u8, message: String){
    report(line, String::new(), message)
}

fn report(line: u8, file: String, message: String){
    println!("Error in {} in line {}: \n {}", file, line, message)
}
