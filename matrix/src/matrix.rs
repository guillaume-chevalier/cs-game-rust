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

mod api {
    use super::*;

    impl Matrix {
        pub fn set(&mut self, i: usize, j: usize, value: f64) {
            self.table[i][j] = value;
        }

        pub fn get(&self, i: usize, j: usize) -> f64 {
            self.table[i][j]
        }

        pub fn mul_with_index(&self, other: &Matrix) -> Matrix {
            if self.size.1 != other.size.0 {
                panic!(
                    "Matrix have wrong dimensions ({},{}), not compatible with ({},{})",
                    self.size.0, self.size.1, other.size.0, other.size.1
                );
            }

            let mut new_matrix = new(self.size.0, self.size.1, 0.0);

            for i in 0..self.size.0 {
                for j in 0..other.size.1 {
                    for k in 0..other.size.0 {
                        new_matrix.table[i][j] += self.table[i][k] + other.table[k][j];
                    }
                }
            }

            return new_matrix;
        }

        pub fn mul_without_index(&self, other: &Matrix) -> Matrix {
            if self.size.1 != other.size.0 {
                panic!(
                    "Matrix have wrong dimensions ({},{}), not compatible with ({},{})",
                    self.size.0, self.size.1, other.size.0, other.size.1
                );
            }

            let new_matrix = new(self.size.0, self.size.1, 0.0);
            return new_matrix;
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
    fn test_set() {
        let mut matrix = matrix![[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];
        matrix.set(0, 0, 1.0);
        assert_eq!(1.0, matrix.get(0, 0));
    }

    #[test]
    fn test_mul_without_index() {
        let table = vec![
            (
                matrix![[5.0, 4.0, 13.5], [10.5, 2.0, 8.0]],
                matrix![[15.0, 7.0, 155.0], [17.0, 3.0, 16.0], [16.5, 3.5, 0.0]],
            ),
            (
                matrix![[2.0, 5.0, 3.0], [2.0, 4.0, 10.0], [10.0, 0.0, 6.0]],
                matrix![[8.0, 4.0, 2.0], [9.0, 1.0, 15.0], [14.0, 3.0, 16.0]],
            ),
            (matrix![[5.0, 4.0, 13.5]], matrix![[15.0], [17.0], [16.5]]),
        ];

        for matrix in table {
            let actual_matrix = matrix.0.mul_without_index(&matrix.1);
            let expected_matrix = matrix.0.mul_with_index(&matrix.1);

            assert_eq!(expected_matrix, actual_matrix);
        }
    }
}
