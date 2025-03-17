use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let result = count(&input);
    println!("{}", result);
}

fn count(input: &str) -> usize {
    input.trim().chars().filter(|&c| c == '1').count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "101\n";
        assert_eq!(count(input), 2);
    }

    #[test]
    fn test2() {
        let input = "000\n";
        assert_eq!(count(input), 0);
    }
}