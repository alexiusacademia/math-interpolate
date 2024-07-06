# math-interpolate

`math-interpolate` is a Rust library that interpolates two points to calculate the third point in between them.

## Usage

```rust
let p1 = Point::new(0.0, 0.0);  // x, y is 0.0, 0.0
let p2 = Point::new(5.0, 5.0);
let x: f32 = 3.0;

let result = interpolate(p1, p2, x);

println!("The value of y is {}.", result);
```