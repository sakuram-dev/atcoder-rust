# Memo

## Read Input

```rust
let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
let mut stdin = stdin.split_whitespace();
let n: i32 = stdin.next().unwrap().parse().unwrap();
let mut v: Vec<i32> = Vec::new();
for _ in 0..n {
    v.push(stdin.next().unwrap().parse().unwrap());
}
```

## Vector

### Sort a Vector of Integers

```rust
v.sort(); // sort in ascending order
v.sort_by(|a, b| a.cmp(b)); // sort in ascending order
v.sort_by(|a, b| b.cmp(a)); // sort in descending order
```

### Remove Duplicates

```rust
v.sort(); // sort in ascending order
v.dedup(); // remove duplicates
```
