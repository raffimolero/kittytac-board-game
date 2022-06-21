use super::*;

mod num_to_char {
    use super::num_to_char;

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

mod arr_2d_from_iter {
    use super::arr_2d_from_iter;

    #[test]
    #[should_panic]
    fn test_not_enough() {
        let mut iter = [1, 2, 3, 4, 5].into_iter();
        let _: [[u8; 2]; 3] = arr_2d_from_iter(&mut iter);
    }

    #[test]
    fn test_just_right() {
        let mut iter = [1, 2, 3, 4, 5, 6].into_iter();
        let arr: [[u8; 2]; 3] = arr_2d_from_iter(&mut iter);
        assert_eq!(arr, [[1, 2], [3, 4], [5, 6]]);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_with_extras() {
        let mut iter = [1, 2, 3, 4, 5, 6, 7].into_iter();
        let arr: [[u8; 2]; 3] = arr_2d_from_iter(&mut iter);
        assert_eq!(arr, [[1, 2], [3, 4], [5, 6]]);
        assert_eq!(iter.next(), Some(7));
    }
}
