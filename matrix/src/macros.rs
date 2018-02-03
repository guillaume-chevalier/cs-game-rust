#[macro_export]
macro_rules! matrix {
    ( $( $x:expr ),* ) => {
        {
            let mut table = Vec::new();
            $(
                let temp_vec = $x.to_vec();
                table.push(temp_vec);
            )*
            matrix::from_vec(table)
        }
    };
}

#[cfg(test)]
mod tests {
    use matrix;

    #[test]
    fn should_construct_expected_matrix() {
        let matrix = matrix![[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];
        let expected_matrix = matrix::new(3, 3, 0.0);
        assert_eq!(expected_matrix, matrix);
    }

    #[test]
    #[should_panic]
    fn should_panic_when_all_sub_vectors_have_different_len() {
        matrix![[0.0, 0.0, 0.0], [0.0, 0.0], [0.0, 0.0, 0.0]];
    }
}
