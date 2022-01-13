use hello_cargo::equals::equals::equals_array;
use hello_cargo::equals::equals::call_nested_equals;
extern crate hello_cargo;
fn main() {
  let a = vec![1, 2, 3];
  let b = vec![1, 2, 3];
  println!("{}", equals_array(a, b));
  call_nested_equals();
}
