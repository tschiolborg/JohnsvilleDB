use rand::Rng;
use std::collections::HashMap;
use std::io;

struct Table {
    name: String,
    rows: Vec<Row>,
}

#[derive(Clone, Debug)]
struct Row {
    fields: HashMap<String, String>,
}

impl Table {
    fn new(name: String) -> Table {
        Table {
            name,
            rows: Vec::new(),
        }
    }

    fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    fn filter(&self, keys: Vec<String>, condition: fn(&Row) -> bool) -> Vec<Row> {
        let mut result: Vec<Row> = Vec::new();
        for row in self.rows.iter() {
            if condition(row) {
                let mut row = row.clone();
                if !keys.is_empty() {
                    let mut fields = HashMap::new();
                    for key in keys.iter() {
                        if let Some(value) = row.fields.get(key) {
                            fields.insert(key.clone(), value.clone());
                        }
                    }
                    row.fields = fields;
                }
                result.push(row);
            }
        }
        result
    }
}

impl Row {
    fn new() -> Row {
        Row {
            fields: HashMap::new(),
        }
    }

    fn add_field(&mut self, key: String, value: String) {
        self.fields.insert(key, value);
    }
}

fn read_query(table_name: &str) -> Option<(Vec<String>, fn(&Row) -> bool)> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut keys: Vec<String> = Vec::new();
    let condition: fn(&Row) -> bool = |_| true;

    let input = input.trim().to_lowercase().replace(",", "");

    let mut tokens = input.split_whitespace();

    match tokens.next() {
        Some("select") => (),
        Some("exit") => std::process::exit(0),
        _ => {
            println!("Invalid query: Missing SELECT");
            return None;
        }
    }

    let mut select_all = false;
    let mut cnt = 0;
    loop {
        match tokens.next() {
            Some("from") => {
                if cnt == 0 {
                    println!("Invalid query: Missing columns to select");
                    return None;
                }
                break;
            }
            Some("*") => {
                if cnt > 0 {
                    println!("Invalid query: Select * can only be used alone.");
                    return None;
                }
                keys.clear();
                select_all = true;
            }
            Some(key) => {
                if select_all {
                    println!("Invalid query: Select * can only be used alone.");
                    return None;
                }
                keys.push(key.to_string())},
            None => {
                println!("Invalid query: Missing FROM");
                return None;
            }
        }
        cnt += 1;
    }

    match tokens.next() {
        Some(name) if name == table_name => (),
        _ => {
            println!("Invalid query: Only table '{table_name}' is supported");
            return None;
        }
    }

    match tokens.next() {
        Some("where") => {
            println!("Invalid query: WHERE is not yet supported");
            return None;
        }
        Some(_) => {
            println!("Invalid query: Only WHERE is supported");
            return None;
        }
        None => return Some((keys, condition)),
    }
}

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
        println!("\nMake query on table '{}'", table.name);

        let (keys, condition) = match read_query(&table.name) {
            Some((keys, condition)) => (keys, condition),
            None => continue,
        };

        let result = table.filter(keys, condition);
        
        for row in result.iter() {
            println!("{:?}", row);
        }
    }
}
