extern crate libloading as dlib;

use std::env;
use std::io::prelude::*;

fn get_puzzle_string(input_string: &str) -> String {

    // Try reading as a file. If it fails then assume argument is the input string as is.
    match std::fs::File::open(input_string) {
        Ok(mut f) => {
            let mut puzzle_string = String::new();
            f.read_to_string(&mut puzzle_string).expect("something went wrong reading the file");
            puzzle_string
        }
        Err(_) => {
            input_string.to_string()
        }
    }
}


fn call_solution(day_number: i32, input: &str) -> dlib::Result<String> {
    let lib_path = format!("target/debug/deps/libaoc2018_day{:02}.so",day_number);
    let dlib = dlib::Library::new(lib_path)?;
    unsafe {
        let func: dlib::Symbol<unsafe extern fn(&str) -> String> = dlib.get(b"aoc_solution")?;
        Ok(func(input))
    }
}


fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        panic!("Please provide the day# followed by the input or a file name");
    }

    match call_solution(args[1].parse().unwrap(),&get_puzzle_string(&args[2])) {
        Ok(solution) => { println!("{}",solution) },
        Err(err) => { panic!(err.to_string()) } 
    }
}
