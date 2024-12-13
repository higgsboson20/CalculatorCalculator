/*
Idea: given eq E: f = f(x0, x1, ... , xn), solve for variable xi

Idea: take input string containing equation and variable to solve for

*/

mod process;
mod gui;

use std::{fs::File, io::{self, BufRead}};
use gui::start_app;
use process::get_vars;

const PATH: &str = "src/testfiles/eqs1.txt";

fn process_file() {
    let file = File::open(PATH).expect("Unable to open the file");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Error reading line");
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() != 2 {
            println!("Invalid format in line: {}", line);
            continue;
        }

        let equation = parts[0].trim().to_string();
        let variable = parts[1].trim().to_string();

        match get_vars(&equation, &variable) {
            Ok(vars) => println!("Variables: {:?}", vars),
            Err(err) => println!("Error: {}", err)
        }

    }

}

fn main() {
    //process_file();
    match start_app() {
        Ok(_) => {},
        Err(err) => print!("Error: {}", err)
    }
}
