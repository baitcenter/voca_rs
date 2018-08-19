extern crate voca_rs;

#[cfg(test)]
mod tests {
    use voca_rs::*;

    #[test]
    fn utils_version() {
        assert_eq!(utils::version(), "0.1.0");
    }

    #[test]
    fn split_to_chars() {
        assert_eq!(
            split::chars(&"gravity".to_string()),
            ["g", "r", "a", "v", "i", "t", "y"]
        );
        assert_eq!(split::chars(&"  ".to_string()), [" ", " "]);
        assert_eq!(split::chars(&"a b".to_string()), ["a", " ", "b"]);
        assert_eq!(split::chars("\n\t"), ["\n", "\t"]);
    }
    #[test]
    #[should_panic]
    fn split_to_chars_panic() {
        assert_eq!(split::chars(&"gravity".to_string()), ["g", "r", "a"]);
    }

    #[test]
    fn split_by_pattern() {
        assert_eq!(
            split::split("gravity can cross dimensions", " "),
            ["gravity", "can", "cross", "dimensions"]
        );
        assert_eq!(split::split("*dying*star*", "*"), ["", "dying", "star"]);
        assert_eq!(split::split("dying star", ""), ["dying star"]);
    }
    #[test]
    #[should_panic]
    fn split_by_pattern_panic() {
        assert_eq!(split::chars(&"gravity".to_string()), ["g", "r", "a"]);
    }

    #[test]
    fn split_words() {
        assert_eq!(
            split::words(&"gravity can cross dimensions".to_string()),
            ["gravity", "can", "cross", "dimensions"]
        );
        assert_eq!(
            split::words(&"gravity    dying\r\nstar\tfalling".to_string()),
            ["gravity", "dying", "star", "falling"]
        );
        assert_eq!(
            split::words(&"Zażółć gęślą jaźń".to_string()),
            ["Zażółć", "gęślą", "jaźń"]
        );
        assert_eq!(
            split::words(&"splitCamelCase".to_string()),
            ["split", "Camel", "Case"]
        );
        assert_eq!(
            split::words(&"split-some kind_of_mixed CaseHere".to_string()),
            ["split", "some", "kind", "of", "mixed", "Case", "Here"]
        );
    }
    #[test]
    #[should_panic]
    fn split_words_panic() {
        assert_eq!(
            split::words(&"gravity can cross dimensions".to_string()),
            ["gravity1", "can", "cross", "dimensions"]
        );
    }

    #[test]
    fn split_to_graphemes() {
        assert_eq!(
            split::graphemes(&"a̐éö̲\r\n".to_string()),
            ["a̐", "é", "ö̲", "\r\n"]
        );
    }
    #[test]
    #[should_panic]
    fn split_to_graphemes_panic() {
        assert_eq!(split::graphemes(&"\r".to_string()), ["r"]);
    }

    #[test]
    fn query_ends_with() {
        assert!(query::ends_with("the world is yours", "is yours"));
        assert!(query::ends_with("Zażółć gęślą jaźń", "jaźń"));
        assert!(query::ends_with("the world is yours", ""));
        assert!(query::ends_with("", ""), true);
    }
    #[test]
    #[should_panic]
    fn query_dont_ends_with() {
        assert!(query::ends_with("a b c", "b"));
    }
    #[test]
    fn query_starts_with() {
        assert!(query::starts_with("the world is yours", "the world"));
        assert!(query::starts_with(
            "Zażółć gęślą jaźń",
            "Zażółć"
        ));
        assert!(query::starts_with("the world is yours", ""));
        assert!(query::starts_with("", ""), true);
    }
    #[test]
    #[should_panic]
    fn query_dont_starts_with() {
        assert!(query::starts_with("a b c", "b"));
    }
}
