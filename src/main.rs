use std::env;

use rustyline::{Editor, Result, error::ReadlineError};

use crate::parser::{numbers0, Expression, test_input};
mod parser;

fn main() -> Result<()>{
    env::set_var("RUST_BACKTRACE", "1");
    let mut rl = Editor::<()>::new()?;
    println!("lispy version: 0.0.1");
    println!("Press Ctrl+c to Exit");
    if rl.load_history("history.txt").is_err() {
        println!("No previous history");
    }

    loop {
        let readline = rl.readline("lispy>> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {:#?}", test_input(&line).unwrap().1)
                // println!("Line: {:#?}", numbers0(&line).unwrap_or(("default", vec![Expression::Number(0)])).1)
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt")
}
