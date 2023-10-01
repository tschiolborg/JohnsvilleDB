use crate::backend::{Row, Statement};

pub fn read_input(query: &str) -> Result<Statement, &str> {
    let query = query.trim();

    match query.split_whitespace().next() {
        Some("exit") => std::process::exit(0),
        Some(_) => (),
        None => return Err("Invalid query: Missing tokens"),
    }

    return prepare_statement(&query);
}

fn prepare_statement(query: &str) -> Result<Statement, &str> {
    let statement: Statement;

    let mut tokens = query.split_whitespace();

    match tokens.next() {
        Some("insert") => {
            let args = tokens.collect::<Vec<&str>>();
            if args.len() != 3 {
                return Err("You must provide 3 arguments as values for insert statement");
            }
            let id = args[0].parse::<u32>().map_err(|_| "Invalid id")?;
            let name = args[1].to_string();
            let age = args[2].parse::<u32>().map_err(|_| "Invalid age")?;

            statement = Statement::Insert(Row::from(id, name, age));
        }
        Some("select") => {
            statement = Statement::Select;
        }
        _ => {
            return Err("Unrecognized keyword at start of query");
        }
    }

    return Ok(statement);
}
