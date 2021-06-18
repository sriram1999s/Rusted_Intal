use intal::binop;
use intal::macros::CompRes;
use intal::processing::intal_remove_leadzeros;
fn main() {
    let intal1 = "100";
    let intal2 = "1";

    // test 1 : ADD
    let res_add = binop::intal_add(intal1, intal2);
    println!("intal1 {}, intal2 {}, res_add {}", intal1, intal2, res_add);

    // test 2 : COMPARE
    let res_comp = binop::intal_compare(intal1, intal2);
    match res_comp {
        CompRes::Greater => println!("Greater!"),
        CompRes::Lesser => println!("Lesser!"),
        CompRes::Equal => println!("Equal!"),
    }

    // test 3 : DIFF
    let res_diff = binop::intal_diff(intal1, intal2);
    println!("intal1 {}, intal2 {}, res_diff {}", intal1, intal2, res_diff.unwrap());

    let some_number = "009";
    let new_number = intal_remove_leadzeros(&some_number);
    println!("some number: {} new number: {}", some_number, new_number);
}
