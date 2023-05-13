# Rust cheatsheet
## A cheatsheet for rust concepts

### Data types:
 - Integers:

| Length  | Signed  | Unsigned |
|---------|---------|----------|
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

 - Float: f32, 64 
 - Tuple:
   - `let tup: (i32, f64, u8) = (500, 6.4, 1);`
   - `let (x, y, z) = tup;` - destructuring a tuple
   - `x.0`, `x.1`, `x.2` - accessing elements of a tuple
 - Array:
   - `let a: [i32; 5] = [1, 2, 3, 4, 5]`
   - Arrays have **fixed length**
   - `let a = [1, 2]` - quick definition
   - `let a = [1; 2]` - length 2, filled with 1s

