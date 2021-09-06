// use crate::equals::equals_array;
// use crate::equals::nested_equals::equals_by_nested_equals;
// use crate::equals::nested_equals::option;
// use crate::equals::call_nested_equals;
extern crate hello_cargo;
fn main() {
    // let a: [u8; 3] = [1, 2, 3];
    // let b: [u8; 3] = [1, 2, 3];
    let a = vec![1, 2, 3];
    let b = vec![1, 2, 3];
    // print!!("{}", equals_array(a, b));
    println!("{}", hello_cargo::equals::equals_array(a, b));
    hello_cargo::equals::call_nested_equals();
}
