use crate::backend::Row;
use std::io;

pub fn read_query(table_name: &str) -> Option<(Vec<String>, fn(&Row) -> bool)> {
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
                keys.push(key.to_string())
            }
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
