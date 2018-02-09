use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct Matrix {
    table: Vec<Vec<f64>>,
    pub size: (usize, usize),
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

    Matrix {
        table: table,
        size: (size_x, size_y),
    }
}

pub fn from_vec(vec: Vec<Vec<f64>>) -> Matrix {
    let size = (vec.len(), vec[0].len());

    for row in &vec {
        if row.len() != size.1 {
            panic!("Error : All sub vectors must have the same len.");
        }
    }

    Matrix {
        table: vec,
        size: size,
    }
}

impl Matrix{
    pub fn add(&mut self, value_to_add: f64){
        for i in 0..self.size.0 {
            for j in 0..self.size.1 {
                self.table[i][j] = self.table[i][j] + value_to_add;
            }
        }
    }
}
impl Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for row in &self.table {
            for item in row {
                output += &format!("{} ", item);
            }
            output += "\n"
        }
        return write!(f, "{}", output);
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        self.table == other.table
    }
}

#[cfg(test)]
mod tests {
    use matrix;

    #[test]
    fn test_add() {
        let table = vec![
            (matrix![[2.0], [1.5]], 2.5, matrix![[4.5], [4.0]]),
            (
                matrix![[2.0, 5.0, 3.0], [2.0, 4.0, 10.0], [10.0, 0.0, 6.0]],
                1.0,
                matrix![[3.0, 6.0, 4.0], [3.0, 5.0, 11.0], [11.0, 1.0, 7.0]],
            ),
            (
                matrix![
                    [2.0, 5.0, 3.0, -9.0],
                    [2.0, 4.0, 10.0, -10.0],
                    [10.0, 0.0, 6.0, 0.0]
                ],
                -1.0,
                matrix![
                    [1.0, 4.0, 2.0, -10.0],
                    [1.0, 3.0, 9.0, -11.0],
                    [9.0, -1.0, 5.0, -1.0]
                ],
            ),
        ];

        for value in table {
            let mut matrix = value.0;
            let value_to_add = value.1;
            let expected_matrix = value.2;

            matrix.add(value_to_add);

            assert_eq!(expected_matrix, matrix);
        }
    }
}
