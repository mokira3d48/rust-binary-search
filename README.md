# Rust Binary Search
An implementation of the binary search algorithm in Rust.

## Usage

```rust
mod binary_search;

fn main() {
    let array_sorted: Vec<i32> = vec![2, 3, 4, 5, 6];
    // We want to find number 5 and 10 in this array:
    let result1 = binary_search::find(array_sorted.as_slice(), 5);
    let result2 = binary_search::find(array_sorted.as_slice(), 10);

    match result1 {
      Some(pos) => println!("The position of {} is: {}", 5, pos),
      None => println!("The number {} is not found.", 5),
    };
    match result2 {
      Some(pos) => println!("The position of {} is: {}", 10, pos),
      None => println!("The number {} is not found.", 10),
    };
}
```
