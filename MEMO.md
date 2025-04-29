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

## String

### ends_with

Check if a string ends with a specific substring (case-sensitive).

```rust
let s = "abcde";
let t = "de";
let result = s.ends_with(t);
println!("{}", result); // true
```

### truncate

Truncate a string to a specified length by removing characters from the end.

```rust
let mut s = "abcde".to_string();
let t = "de";
if s.ends_with(t) {
    s.truncate(s.len() - t.len());
}
println!("{}", s); // abc
```