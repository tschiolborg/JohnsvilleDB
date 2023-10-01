mod backend;
mod parser;

use rand::Rng;

use backend::Row;
use backend::Table;
use parser::read_query;

fn main() {
    let mut rnd = rand::thread_rng();

    let mut table = Table::new(String::from("users"));

    for i in 1..=3 {
        let mut row = Row::new();
        row.add_field(String::from("id"), i.to_string());
        row.add_field(String::from("name"), format!("user{}", i));
        row.add_field(String::from("age"), rnd.gen_range(20..=80).to_string());
        table.add_row(row);
    }

    loop {
        println!("\nMake query on table '{}'", table.name());

        let (keys, condition) = match read_query(&table.name()) {
            Some((keys, condition)) => (keys, condition),
            None => continue,
        };

        let result = table.filter(keys, condition);

        for row in result.iter() {
            println!("{:?}", row);
        }
    }
}
