use std::collections::HashMap;

pub const TABLE_NAME: &str = "users";

pub struct Table {
    #[allow(dead_code)]
    name: String,
    rows: Vec<Row>,
}

#[derive(Clone, Debug)]
pub struct Row {
    fields: HashMap<String, String>,
}

impl Table {
    pub fn new(name: String) -> Table {
        Table {
            name,
            rows: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    pub fn filter(&self, keys: Vec<String>, condition: fn(&Row) -> bool) -> Vec<Row> {
        let mut result: Vec<Row> = Vec::new();
        for row in self.rows.iter() {
            if condition(row) {
                let mut new_row = Row::new();
                if keys.is_empty() {
                    for (key, value) in row.fields.iter() {
                        new_row.add_field(key.to_string(), value.to_string());
                    }
                } else {
                    for key in keys.iter() {
                        new_row.add_field(key.to_string(), row.fields[key].to_string());
                    }
                }
                result.push(new_row);
            }
        }
        result
    }
}

impl Row {
    pub fn new() -> Row {
        Row {
            fields: HashMap::new(),
        }
    }

    pub fn add_field(&mut self, key: String, value: String) {
        self.fields.insert(key, value);
    }
}
