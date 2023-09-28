#[cfg(test)]
mod tests {
    use crate::can_make_word;

    #[test]
    fn test_word() {
        assert_eq!(can_make_word("A".to_string()), true);
        assert_eq!(can_make_word("BARK".to_string()), true);
        assert_eq!(can_make_word("BOOK".to_string()), false);
        assert_eq!(can_make_word("TREAT".to_string()), true);
        assert_eq!(can_make_word("COMMON".to_string()), false);
        assert_eq!(can_make_word("SQUAD".to_string()), true);
        assert_eq!(can_make_word("CONFUSE".to_string()), true);
    }
}
