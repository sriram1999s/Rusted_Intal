// Library "intal" - Integer of arbitray length
// intal is a nonnegative integer of arbitrary length (not more than 1000 decimal digits).
// The integer is stored as a null terminated string of ASCII characters.
// String of decimal digits ('0' thru '9') are stored in big endian style.
// That is, the most significant digit is at the head of the string.
// Eg: Integer 25 is stored in str as '2' at str[0], '5' at str[1], and null char at str[2].
pub mod macros {
    pub const RADIX: u32 = 10;
    pub enum CompRes {
        Greater,
        Lesser,
        Equal,
    }
}
pub mod binop {
    // To eliminate leading zeros
    fn intal_remove_leadzeros(intal: &str) -> String {
        let mut index = 0;
        let size = intal.len();
        let s = intal.as_bytes();
        while index < size - 1 {
            let c: char = s[index] as char;
            let d = c.to_digit(crate::macros::RADIX).unwrap();
            if d != 0 {
                break;
            }
            index += 1;
        }
        String::from(&intal[index..])
    }
    // Returns the sum of two intals.
    pub fn intal_add(intal1: &str, intal2: &str) -> String {
        let s1 = intal1.chars().rev();
        let s2 = intal2.chars().rev();
        let mut res: String = String::from("");
        let mut carry: u32 = 0;
        let mut index: usize = 0;

        for pair in s1.zip(s2) {
            let (c1, c2) = pair;
            let d1 = c1.to_digit(crate::macros::RADIX).unwrap();
            let d2 = c2.to_digit(crate::macros::RADIX).unwrap();
            // println!("d1: {} d2: {} carry: {}", d1, d2, carry);
            let sum = (d1 + d2 + carry) % 10;
            carry = (d1 + d2 + carry) / 10;
            // println!("{} {}", sum, carry);
            res.push(char::from_digit(sum, crate::macros::RADIX).unwrap());
            index += 1;
        }
        // taking care of extra digits
        if intal1.len() > intal2.len() {
            let s: String = intal1.chars().rev().collect();
            let rem = &s[index..];
            for c in rem.chars() {
                let d = c.to_digit(crate::macros::RADIX).unwrap();
                let sum = (d + carry) % 10;
                carry = (d + carry) / 10;
                // println!("{} {}", sum, carry);
                res.push(char::from_digit(sum, crate::macros::RADIX).unwrap());
            }

        } else if intal1.len() < intal2.len() {
            let s: String = intal2.chars().rev().collect();
            let rem = &s[index..];
            for c in rem.chars() {
                let d = c.to_digit(crate::macros::RADIX).unwrap();
                let sum = (d + carry) % 10;
                carry = (d + carry) / 10;
                // println!("{} {}", sum, carry);
                res.push(char::from_digit(sum, crate::macros::RADIX).unwrap());
            }

        }
        if carry != 0 {
            res.push(char::from_digit(carry, crate::macros::RADIX).unwrap());
        }
        // println!("{}", res);
        let res: String = res.chars().rev().collect();
        // String::from("test")
        res
    }

    // Returns the comparison value of two intals.
    pub fn intal_compare(intal1: &str, intal2: &str) -> crate::macros::CompRes {
        let intal1_len = intal1.len();
        let intal2_len = intal2.len();

        if intal1_len != intal2_len { // case that the intal lengths aren't the same => short circuit decision
            if intal1_len > intal2_len {
                return crate::macros::CompRes::Greater
            } else {
                return crate::macros::CompRes::Lesser
            }
        } else {
            let s1 = intal1.chars();
            let s2 = intal2.chars();
            for pair in s1.zip(s2) {
                let (c1, c2) = pair;
                let d1 = c1.to_digit(crate::macros::RADIX).unwrap();
                let d2 = c2.to_digit(crate::macros::RADIX).unwrap();
                if d1 > d2 {
                    return crate::macros::CompRes::Greater
                } else if d1 < d2 {
                    return crate::macros::CompRes::Lesser
                }
            }
        }
        crate::macros::CompRes::Equal
    }

    // Returns the Option<difference_str> (obviously, nonnegative) of two intals.
    // Return None if results in negative
    pub fn intal_diff(intal1: &str, intal2: &str) -> Option<String> {
        let res_comp = crate::binop::intal_compare(intal1, intal2);
        if let crate::macros::CompRes::Lesser = res_comp {
            return None
        }
        let s1 = intal1.chars().rev();
        let s2 = intal2.chars().rev();
        let mut res: String = String::from("");
        let mut borrow: u32 = 0;
        let mut index: usize = 0;

        for pair in s1.zip(s2) {
            let (c1, c2) = pair;
            let d1 = c1.to_digit(crate::macros::RADIX).unwrap();
            let d2 = c2.to_digit(crate::macros::RADIX).unwrap();
            // println!("d1: {} d2: {} borrow: {}", d1, d2, borrow);
            let temp = borrow;
            let difference;
            if d1 - temp < d2 {
                borrow = 1;
                difference = 10 + (d1 - temp) - d2;
            } else {
                borrow = 0;
                difference = d1 - temp - d2;
            }
            // println!("{} {}", difference, borrow);
            res.push(char::from_digit(difference, crate::macros::RADIX).unwrap());
            index += 1;
        }
        // taking care of extra digits
        if intal1.len() > intal2.len() {
            let s: String = intal1.chars().rev().collect();
            let rem = &s[index..];
            for c in rem.chars() {
                let d = c.to_digit(crate::macros::RADIX).unwrap();
                let temp = borrow;
                let difference;

                // println!("{} {} {}", res, d, borrow);
                if d < temp  {
                    difference = 10 +  d - temp;
                    borrow = 1;
                } else {
                    difference = d - temp;
                }
                res.push(char::from_digit(difference, crate::macros::RADIX).unwrap());
            }

        }
        let res: String = res.chars().rev().collect();
        // String::from("test")
        // let ix = ;
        Some(intal_remove_leadzeros(&res[..]))
    }
}
