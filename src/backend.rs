use core::fmt;

pub enum Meta {
    Exit,
}

pub enum Statement {
    Insert(Row),
    Select,
    Meta(Meta),
}

pub struct Table {
    rows: Vec<Row>,
}

#[derive(Clone, Debug)]
pub struct Row {
    id: u32,
    name: String,
    age: u32,
}

impl Table {
    pub fn new() -> Table {
        Table { rows: Vec::new() }
    }

    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    pub fn filter(&self, condition: fn(&Row) -> bool) -> Vec<Row> {
        let mut result: Vec<Row> = Vec::new();
        for row in self.rows.iter() {
            if condition(row) {
                result.push(row.clone());
            }
        }
        result
    }
}

impl Row {
    pub fn from(id: u32, name: String, age: u32) -> Row {
        Row { id, name, age }
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>3}, {:>4}, {:>3}", self.id, self.name, self.age)
    }
}

pub fn execute_statement(statement: Statement, table: &mut Table) {
    match statement {
        Statement::Meta(meta) => execute_meta(meta),
        Statement::Insert(insert_row) => {
            table.add_row(insert_row);
        }
        Statement::Select => {
            let result = table.filter(|_| true);
            println!(" id, name, age");
            for row in result.iter() {
                println!("{}", row);
            }
        }
    }
}

pub fn execute_meta(meta: Meta) {
    match meta {
        Meta::Exit => std::process::exit(0),
    }
}
