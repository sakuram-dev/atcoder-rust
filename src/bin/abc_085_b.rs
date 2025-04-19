fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();

    let n: i32 = stdin.next().unwrap().parse().unwrap();

    let mut a: Vec<i32> = Vec::new();
    for _ in 0..n {
        let x: i32 = stdin.next().unwrap().parse().unwrap();
        a.push(x);
    };

    println!("{}", kagami_mochi(&mut a));
}

fn kagami_mochi(a: &mut Vec<i32>) -> i32 {
    a.sort();
    a.dedup();
    a.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut a = vec![10, 8, 8, 6];
        assert_eq!(kagami_mochi(&mut a), 3);
    }

    #[test]
    fn test2() {
        let mut a = vec![15, 15, 15];
        assert_eq!(kagami_mochi(&mut a), 1);
    }

    #[test]
    fn test3() {
        let mut a = vec![50, 30, 50, 100, 50, 80, 30];
        assert_eq!(kagami_mochi(&mut a), 4);
    }
}