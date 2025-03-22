# Memo

## Read Input

```rust
let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
let mut stdin = stdin.split_whitespace();
let n: i32 = stdin.next().unwrap().parse().unwrap();
let mut v: Vec<i32> = Vec::new();
for _ in 0..a {
    v.push(stdin.next().unwrap().parse().unwrap());
}
```
