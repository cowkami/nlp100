fn word_length_list(sentence: String) -> Vec<i32> {
    sentence
        .split(" ")
        .map(|word: &str| word.trim_matches(|c: char| c.is_ascii_punctuation()).len() as i32)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::word_length_list;

    #[test]
    fn test_word_length_list() {
        let input_words = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.".to_string();
        let expected = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9];
        let result = word_length_list(input_words);
        assert_eq!(result, expected);
    }
}
