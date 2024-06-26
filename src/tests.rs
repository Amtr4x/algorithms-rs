#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_fizz_buzz() {
        let seq = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let correct_result = vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz",
        ];
        let result = fizz_buzz(seq);
        assert_eq!(result, correct_result)
    }
}
