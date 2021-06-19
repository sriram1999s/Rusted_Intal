# Rusted Intal
## A rust library crate for operations on integers of arbitrary length

- **Intal** => is a nonnegative integer of arbitrary length.
- It is stored as a string in Big Endian Format, used to store and compute on very large integers.
- This crate provides various functionalities for such computations.

## Functionalities

Processing operations:
  - [intal_remove_leadzeros](#Eliminate-leading-zeros)

Binary Operations:
  - [intal_add](#Addition)
  - [intal_compare](#Comparison)
  - [intal_diff](#Subtraction)
  - [intal_multiply](#Multiplication)
  - [intal_pow](#Exponentiation)
## Inclusion

To include defined types:
```rust
use intal::def::<name_of_type>;
// use intal::def::*; // for all public types
```
Type list provided [here](#Types)

To include processing operations:
```rust
use intal::processing::<name_of_>;
// use intal::processing::*; // for all public operations
```

To include binary operations:
```rust
use intal::binop::<name_of_op>;
// use intal::binop::*; // for all public operations
```

## Usage

### Eliminate leading zeros

By convention, intals must not have leading zeros. The function ```intal_remove_leadzeros``` is provided to the user to achieve the same.

Takes ```&str``` as parameter and returns a ```String```

```rust
use intal::processing::intal_remove_leadzeros;
//...
let some_number = "009";
let new_number = intal_remove_leadzeros(&some_number); // 9
```

### Addition

Function ```intal_add``` is provided to add two intals. It takes two parameters, both of type ```&str```. It returns a ```String```.

```rust
use intal::binop;
//...
let intal1 = "267458350436957867";
let intal2 = "32784692498348";
let res_add = binop::intal_add(intal1, intal2);
```

### Comparison

Function ```intal_compare``` is provided to compare two intals. It takes two parameters, both of type ```&str```. It returns an instance of enum ```CompRes```. Enums and their details can be found under the Types section [here](#Types)

```rust
use intal::binop;
//...
let intal1 = "267458350436957867";
let intal2 = "32784692498348";
let res_comp = binop::intal_compare(intal1, intal2);
match res_comp {
    CompRes::Greater => println!("Greater!"),
    CompRes::Lesser => println!("Lesser!"),
    CompRes::Equal => println!("Equal!"),
}
```

### Subtraction

Function ```intal_diff``` is provided to subtract two intals. It takes two parameters, both of type ```&str```. Since intals are nonnegative, the function returns ```Option<String>```, ```Some(string)``` if difference is nonnegative and  ```None``` otherwise.

```rust
use intal::binop;
//...
let intal1 = "267458350436957867";
let intal2 = "32784692498348";
let res_diff = binop::intal_diff(intal1, intal2);
match res_diff {
  Some(number) => println!("The difference is {}",number),
  None => println!("Oops!Nonnegative are not valid"),
}
```
### Multiplication

Function ```intal_multiply``` is provided to add two intals. It takes two parameters, both of type ```&str```. It returns a ```String```.

```rust
use intal::binop;
//...
let intal1 = "267458350436957867";
let intal2 = "32784692498348";
let res_mul = binop::intal_multiply(intal1, intal2);
```

### Exponentiation

Function ```intal_pow``` is provided for calculating powers. It takes two parameters, both of type ```&str```. It returns ```Option<String>```, it will return ```Some(string)``` for all cases except the 0^0 case, for which it will return ```None```.

```rust
let num = "2";
let pow = "3";
let res_pow = binop::intal_pow(num, pow);
```

## Types

### CompRes
It is an enum used to return the result of a comparison.
```rust
pub enum CompRes {
      Greater,
      Lesser,
      Equal,
  }
```

### RADIX
Defines a base for char to int conversion.
```rust
pub const RADIX: u32 = 10;
```
