fn main () {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();

    let n: i32 = stdin.next().unwrap().parse().unwrap();

    let mut a: Vec<i32> = Vec::new();
    for _ in 0..n {
        let x: i32 = stdin.next().unwrap().parse().unwrap();
        a.push(x);
    };

    println!("{}", calculate_score_difference(n, &mut a));
}

fn calculate_score_difference(n: i32, a: &mut Vec<i32>) -> i32 {
    let mut score = 0;
    
    // Sort the array in descending order
    a.sort_by(|a, b| b.cmp(a));
    
    for i in 0..n {
        if i % 2 == 0 {
            score += a[i as usize];
        } else {
            score -= a[i as usize];
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 2;
        let mut a = vec![3, 1];
        assert_eq!(calculate_score_difference(n, &mut a), 2);
    }

    #[test]
    fn test2() {
        let mut a = vec![2, 7, 4];
        let n = 3;
        assert_eq!(calculate_score_difference(n, &mut a), 5);
    }

    #[test]
    fn test3() {
        let mut a = vec![20, 18, 2, 18];
        let n = 4;
        assert_eq!(calculate_score_difference(n, &mut a), 18);
    }
}