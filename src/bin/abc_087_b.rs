fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();

    let a: i32 = stdin.next().unwrap().parse().unwrap();
    let b: i32 = stdin.next().unwrap().parse().unwrap();
    let c: i32 = stdin.next().unwrap().parse().unwrap();
    let x: i32 = stdin.next().unwrap().parse().unwrap();

    println!("{}", count(a, b, c, x));
}

fn count(a: i32, b: i32, c: i32, x: i32) -> i32 {
    let mut count = 0;

    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                if 500 * i + 100 * j + 50 * k == x {
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let a = 2;
        let b = 2;
        let c = 2;
        let x = 100;
        assert_eq!(count(a, b, c, x), 2);
    }

    #[test]
    fn test2() {
        let a = 5;
        let b = 1;
        let c = 0;
        let x = 150;
        assert_eq!(count(a, b, c, x), 0);
    }

    #[test]
    fn test3() {
        let a = 30;
        let b = 40;
        let c = 50;
        let x = 6000;
        assert_eq!(count(a, b, c, x), 213);
    }
}