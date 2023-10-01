mod backend;
mod parser;

use rand::Rng;
use std::io::{self, Write};

use backend::{Row, Table, TABLE_NAME};
use parser::read_query;

fn main() {
    let mut rnd = rand::thread_rng();

    let mut table = Table::new(String::from(TABLE_NAME));

    for i in 1..=3 {
        let mut row = Row::new();
        row.add_field(String::from("id"), i.to_string());
        row.add_field(String::from("name"), format!("user{}", i));
        row.add_field(String::from("age"), rnd.gen_range(20..=80).to_string());
        table.add_row(row);
    }

    loop {
        print!("> ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let (keys, condition) = match read_query(&input) {
            Some((keys, condition)) => (keys, condition),
            None => continue,
        };

        let result = table.filter(keys, condition);

        for row in result.iter() {
            println!("{:?}", row);
        }
    }
}
