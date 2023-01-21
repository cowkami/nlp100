fn extract_odd_chars(s: String) -> String {
    s.chars().step_by(2).collect()
}

#[cfg(test)]
mod tests {
    use super::extract_odd_chars;

    #[test]
    fn test_extract_odd_chars() {
        let input_string = "パタトクカシーー".to_string();
        let expected = "パトカー".to_string();
        let actuall = extract_odd_chars(input_string);
        assert_eq!(actuall, expected);
    }
}
