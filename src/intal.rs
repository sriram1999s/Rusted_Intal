// Library "intal" - Integer of arbitray length
// intal is a nonnegative integer of arbitrary length (not more than 1000 decimal digits).
// The integer is stored as a null terminated string of ASCII characters.
// String of decimal digits ('0' thru '9') are stored in big endian style.
// That is, the most significant digit is at the head of the string.
// Eg: Integer 25 is stored in str as '2' at str[0], '5' at str[1], and null char at str[2].
pub mod macros {
    pub const RADIX: u32 = 10;
}
pub mod binop {
    // Returns the sum of two intals.
    pub fn intal_add(intal1: &str, intal2: &str) -> String {
        let s1 = intal1.chars().rev();
        let s2 = intal2.chars().rev();
        let mut res: String = String::from("");
        let mut carry: u32 = 0;
        let mut index: usize = 0;

        for pair in s1.zip(s2) {
            let (c1, c2) = pair;
            let d1 = c1.to_digit(super::macros::RADIX).unwrap();
            let d2 = c2.to_digit(super::macros::RADIX).unwrap();
            // println!("d1: {} d2: {} carry: {}", d1, d2, carry);
            let sum = (d1 + d2 + carry) % 10;
            carry = (d1 + d2 + carry) / 10;
            // println!("{} {}", sum, carry);
            res.push(char::from_digit(sum, super::macros::RADIX).unwrap());
            index += 1;
        }
        if intal1.len() > intal2.len() {
            let s: String = intal1.chars().rev().collect();
            let rem = &s[index..];
            for c in rem.chars() {
                let d = c.to_digit(super::macros::RADIX).unwrap();
                let sum = (d + carry) % 10;
                carry = (d + carry) / 10;
                // println!("{} {}", sum, carry);
                res.push(char::from_digit(sum, super::macros::RADIX).unwrap());
            }

        } else if intal1.len() < intal2.len() {
            let s: String = intal2.chars().rev().collect();
            let rem = &s[index..];
            for c in rem.chars() {
                let d = c.to_digit(super::macros::RADIX).unwrap();
                let sum = (d + carry) % 10;
                carry = (d + carry) / 10;
                // println!("{} {}", sum, carry);
                res.push(char::from_digit(sum, super::macros::RADIX).unwrap());
            }

        }
        if carry != 0 {
            res.push(char::from_digit(carry, super::macros::RADIX).unwrap());
        }
        // println!("{}", res);
        let res: String = res.chars().rev().collect();
        // String::from("test")
        res
    }
}
