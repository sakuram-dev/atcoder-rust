use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let result = determine_even_or_odd(&input);
    println!("{}", result);
}

fn determine_even_or_odd(input: &str) -> &str {
    let mut numbers = input.split_whitespace();
    let a: i32 = numbers.next().unwrap().parse().expect("Not a number");
    let b: i32 = numbers.next().unwrap().parse().expect("Not a number");

    if (a * b) % 2 == 0 {
        "Even"
    } else {
        "Odd"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abc_086_1() {
        let input = "3 4\n";
        assert_eq!(determine_even_or_odd(input), "Even");
    }

    #[test]
    fn test_abc_086_2() {
        let input = "1 21\n";
        assert_eq!(determine_even_or_odd(input), "Odd");
    }
}