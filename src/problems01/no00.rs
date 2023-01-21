fn reverse_string(s: String) -> String {
    fn f(x: Vec<char>) -> Vec<char> {
        match x.len() {
            0 => vec![],
            1 => vec![x[0]],
            _ => [f(x[1..].to_vec()), vec![x[0]]].concat(),
        }
    }
    f(s.chars().collect()).into_iter().collect::<String>()
}

fn reverse_string_rustish(s: String) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::reverse_string;
    use super::reverse_string_rustish;

    #[test]
    fn test_reverese_string() {
        let input_string = "stressed".to_string();
        let expected = "desserts".to_string();
        let actuall = reverse_string(input_string);
        assert_eq!(actuall, expected);
    }

    #[test]
    fn test_reverese_string_rustish() {
        let input_string = "stressed".to_string();
        let expected = "desserts".to_string();
        let actuall = reverse_string_rustish(input_string);
        assert_eq!(actuall, expected);
    }
}
