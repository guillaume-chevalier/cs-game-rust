mod matrix;

#[cfg(test)]
mod tests {
    use matrix;

    #[test]
    fn test_mul() {
        let matrix = matrix::new(2, 2, 0.0);
        println!("{}", matrix);
    }
}
