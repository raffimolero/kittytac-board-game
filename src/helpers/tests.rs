use super::*;

mod num_to_char {
    use super::*;

    #[test]
    #[should_panic = "10 was too large to convert into a char between '0'..='9'"]
    fn test_num_to_char_overflow_decimal() {
        num_to_char(10, '0'..='9');
    }

    #[test]
    #[should_panic = "27 was too large to convert into a char between 'a'..='z'"]
    fn test_num_to_char_overflow_lowercase_letters() {
        num_to_char(27, 'a'..='z');
    }
}
