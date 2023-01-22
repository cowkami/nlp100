use std::collections::HashMap;

fn extract_elements(s: &str) -> HashMap<&str, i32> {
    let one_char = [1, 5, 6, 7, 8, 9, 15, 16, 19];
    HashMap::from_iter(
        s.split(" ")
            .enumerate()
            .map(|(i, word)| {
                (
                    word.get(if one_char.contains(&(i as i32 + 1)) {
                        0..1
                    } else {
                        0..2
                    })
                    .unwrap(),
                    i as i32 + 1,
                )
            })
            .collect::<Vec<(&str, i32)>>(),
    )
}

#[cfg(test)]
mod tests {
    use super::extract_elements;
    use std::collections::HashMap;

    #[test]
    fn test_extract_elements() {
        let input_sentence = "Hi He Lied Because Boron Could \
        Not Oxidize Fluorine. New Nations Might Also Sign Peace \
        Security Clause. Arthur King Can."
            .to_string();
        let expected = HashMap::from([
            ("H", 1),
            ("He", 2),
            ("Li", 3),
            ("Be", 4),
            ("B", 5),
            ("C", 6),
            ("N", 7),
            ("O", 8),
            ("F", 9),
            ("Ne", 10),
            ("Na", 11),
            ("Mi", 12),
            ("Al", 13),
            ("Si", 14),
            ("P", 15),
            ("S", 16),
            ("Cl", 17),
            ("Ar", 18),
            ("K", 19),
            ("Ca", 20),
        ]);
        let result = extract_elements(&input_sentence);
        assert_eq!(result, expected);
    }
}
