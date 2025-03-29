fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();

    let n: usize = stdin.next().unwrap().parse().unwrap();
    let a: usize = stdin.next().unwrap().parse().unwrap();
    let b: usize = stdin.next().unwrap().parse().unwrap();

    println!("{}", count(n, a, b));
}

fn count(n: usize, a: usize, b: usize) -> usize {
    (1..=n)
        .filter(|&i| {
            let digit_sum: usize = i.to_string().chars().map(|c| c.to_digit(10).unwrap() as usize).sum();
            digit_sum >= a && digit_sum <= b
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 20;
        let a = 2;
        let b = 5;
        assert_eq!(count(n, a, b), 84);
    }

    #[test]
    fn test2() {
        let n = 10;
        let a = 1;
        let b = 2;
        assert_eq!(count(n, a, b), 13);
    }

    #[test]
    fn test3() {
        let n = 100;
        let a = 4;
        let b = 16;
        assert_eq!(count(n, a, b), 4554);
    }
}