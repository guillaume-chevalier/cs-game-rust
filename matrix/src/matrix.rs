use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct Matrix {
    table: Vec<Vec<f64>>,
}

pub fn new(size_x: usize, size_y: usize, default_value: f64) -> Matrix {
    let mut table = Vec::with_capacity(size_x);
    for _ in 0..size_x {
        let mut row = Vec::with_capacity(size_y);
        for _ in 0..size_y {
            row.push(default_value);
        }
        table.push(row);
    }
    Matrix { table: table }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for row in &self.table {
            for item in row {
                output += &format!("{}", item);
            }
            output += "\n"
        }
        return write!(f, "{}", output);
    }
}
