fn fizzbuzz(numbers: Vec<i8>) -> String {
    let mut fizzbuzz_string: String = String::new();
    for number in numbers {
        let bizz_fuzz = if number % 3 == 0 && number % 5 == 0 {
            "FizzBuzz".to_string()
        } else if number % 3 == 0 {
            "Fizz".to_string()
        } else if number % 5 == 0 {
            "Buzz".to_string()
        } else {
            let string: String = number.to_string();
            string
        };
        fizzbuzz_string.push_str(bizz_fuzz.as_str());
        fizzbuzz_string.push_str(" ");
    }
    return fizzbuzz_string;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_return_number() {
        assert_eq!(fizzbuzz(vec![1]), "1 ");
        assert_eq!(fizzbuzz(vec![1, 2]), "1 2 " );
    }

    #[test]
    fn should_fizz() {
        assert_eq!(fizzbuzz(vec![3]), "Fizz ");
        assert_eq!(fizzbuzz(vec![1, 2, 3]), "1 2 Fizz ");
    }

    #[test]
    fn should_buzz() {
        assert_eq!(fizzbuzz(vec![5]), "Buzz ");
        assert_eq!(fizzbuzz(vec![1, 2, 3, 4, 5]), "1 2 Fizz 4 Buzz ");
    }

    #[test]
    fn should_fizz_buzz() {
        assert_eq!(fizzbuzz(vec![15]), "FizzBuzz ");
    }
}