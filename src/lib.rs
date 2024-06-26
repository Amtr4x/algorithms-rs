mod tests;

/// Iterate over a vector and if the number is divisible by 3 it will return Fizz
/// if instead is divisible by 5 it will return Buzz, and if the number is divisible by
/// 3 and 5 it will return FizzBuzz. If the number is not divisible by 3 or 5 will be returned
/// the number.
///
/// ## Arguments
/// * `arr: Vec<u32>` a vector to iterate.
///
/// ## Return
/// * `Vec<String>` a record of the algorithm result.
pub fn fizz_buzz(arr: Vec<u32>) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    arr.iter().for_each(|x| {
        if x % 15 == 0 {
            result.push("FizzBuzz".to_string());
        } else if x % 3 == 0 {
            result.push("Fizz".to_string());
        } else if x % 5 == 0 {
            result.push("Buzz".to_string());
        } else {
            result.push(x.to_string());
        }
    });

    result
}
