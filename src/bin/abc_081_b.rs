fn main(){
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();

    let n: usize = stdin.next().unwrap().parse().unwrap();

    let mut a: Vec<usize> = Vec::new();
    for _ in 0..n {
        a.push(stdin.next().unwrap().parse().unwrap());
    }

   println!("{}", count(a));
}

fn count(mut a: Vec<usize>) -> i32 {
    let mut count = 0;
    while a.iter().all(|&x| x % 2 == 0) {
        a.iter_mut().for_each(|x| *x /= 2);
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let _n = 4;
        let a = vec![5, 6, 8, 10];
        assert_eq!(count(a), 0);
    }

    #[test]
    fn test2() {
        let _n = 6;
        let a = vec![382253568, 723152896, 37802240, 379425024, 404894720, 471526144];
        assert_eq!(count(a), 8);
    }
}
