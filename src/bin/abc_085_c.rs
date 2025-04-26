fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();
    let n: i32 = stdin.next().unwrap().parse().unwrap();
    let y: i32 = stdin.next().unwrap().parse().unwrap();

    let (i, j, k) = calculate(n, y);
    println!("{} {} {}", i, j, k);
}

fn calculate(n: i32, y: i32) -> (i32, i32, i32) {
    for i in 0..=n {
        for j in 0..=(n - i) {
            let k = n - i - j;
            if 10000 * i + 5000 * j + 1000 * k == y {
                return (i, j, k);
            }
        }
    }
    (-1, -1, -1)
}

// Test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(calculate(20, 196000), (-1, -1, -1));
    }

    #[test]
    fn test_2() {
        assert_eq!(calculate(9, 45000), (4, 0, 5));
        
    }
    #[test]
    fn test_3() {
        assert_eq!(calculate(1000, 1234000), (14, 27, 959));
    }
    #[test]
    fn test_4() {
        assert_eq!(calculate(2000, 20000000), (2000, 0, 0));
    }
}
