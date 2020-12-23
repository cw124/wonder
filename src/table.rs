//! Makes it easier to print text-based tables with lined-up columns.

pub struct Table {
    rows: Vec<Vec<String>>,
    num_columns: usize,
}

impl Table {
    /// Creates a new [Table]. Each column will have the given header text.
    pub fn new(header: Vec<String>) -> Table {
        let num_columns = header.len();
        let underlines = header.iter()
            .map(|column_title| "=".repeat(column_title.len()))
            .collect();
        Table {
            rows: vec![header, underlines],
            num_columns
        }
    }

    /// Adds a new row.
    pub fn add(&mut self, row: Vec<String>) {
        if row.len() != self.num_columns {
            panic!("Row has wrong number of columns");
        }
        self.rows.push(row)
    }

    /// Prints the table to stdout. Each line will be prefixed with the given string, and each column will be separated
    /// by the given amount of whitespace.
    pub fn print(&self, prefix: &str, column_spacing: u32) {
        let widths: Vec<usize> = self.rows[0].iter().enumerate()
            .map(|(i, _)| self.rows.iter().map(|row| row[i].len()).max().unwrap_or(0) + column_spacing as usize)
            .collect();

        for row in &self.rows {
            print!("{}", prefix);
            for (i, cell) in row.iter().enumerate() {
                print!("{:width$}", cell, width=widths[i])
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Row has wrong number of columns")]
    fn add_panics_if_wrong_number_of_columns() {
        let mut table = Table::new(vec![String::from("col1")]);
        table.add(vec![])
    }
}
