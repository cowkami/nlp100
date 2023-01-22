use std::iter::zip;

fn join_alternatively(s1: String, s2: String) -> String {
    zip(s1.chars(), s2.chars()).fold("".to_string(), |acc, (c1, c2)| format!("{acc}{c1}{c2}"))
}

#[cfg(test)]
mod tests {
    use super::join_alternatively;

    #[test]
    fn test_join_alterenatively() {
        let input_string1 = "パトカー".to_string();
        let input_string2 = "タクシー".to_string();
        let expected = "パタトクカシーー";
        let result = join_alternatively(input_string1, input_string2);
        assert_eq!(result, expected);
    }
}
