fn reverse_string(s: String) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::reverse_string;

    #[test]
    fn test_reverese_string() {
        let input_string = "stressed".to_string();
        let expected = "desserts".to_string();
        let actuall = reverse_string(input_string);
        assert_eq!(actuall, expected);
    }
}
