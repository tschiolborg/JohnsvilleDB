mod backend;
mod parser;

use std::io::{self, Write};

use backend::{execute_statement, Table};
use parser::read_input;

fn main() {
    let mut table = Table::new();

    loop {
        print!("> ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match read_input(&input) {
            Ok(statement) => {
                execute_statement(statement, &mut table);
            }
            Err(e) => {
                println!("Invalid query: {e}");
                continue;
            }
        }
    }
}
