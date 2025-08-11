# Generics in Rust — Pythagorean Theorem Example

This project is part of a Rust learning journey, focusing on **generics**.
We’ll start with a basic implementation of the Pythagorean theorem using fixed number types (`f64`) and then see why that approach is limiting.
From there, we’ll introduce **generics** to make our code type-flexible, allowing it to work with multiple numeric types like `f32` and `f64`.

---

## 📚 Learning Goals
- Understand what **generics** are and why Rust uses them.
- Learn how to write functions that work for **multiple number types**.
- Use the [`num-traits`](https://docs.rs/num-traits/) crate to provide common numeric operations across different number types.
- Recognize type inference defaults in Rust (e.g., decimals default to `f64`).
- See how generics help reduce code duplication.

---

## 📝 Example Concept

**Naïve implementation (fixed `f64`):**
```rust
fn solve(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}

fn main() {
    println!("{}", solve(3.0, 4.0)); // Output: 5
}
