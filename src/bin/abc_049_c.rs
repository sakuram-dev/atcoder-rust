fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();

    let s: String = stdin.next().unwrap().parse().unwrap();

    println!("{}", can_construct(&s));
}

fn can_construct(s: &str) -> &str {
    let mut s = s.to_string();
    let words = ["dream", "dreamer", "erase", "eraser"];

    loop {
        if s.is_empty() {
            return "YES";
        }

        if let Some(word) = words.iter().find(|&&word| s.ends_with(word)) {
            s.truncate(s.len() - word.len());
        } else {
            return "NO";
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(can_construct("erasedream"), "YES");
        assert_eq!(can_construct("dreameraser"), "YES");
        assert_eq!(can_construct("dreamerer"), "NO");
    }
}
