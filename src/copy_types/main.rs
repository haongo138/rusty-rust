/**
* Copy types
* 1. Integers
* 2. Floats
* 3. Char
* 4. Boolean
* -> Stored in stack -> easy for compiler to copy
**/

fn increase(mut x: i32) -> i32 {
  x += 1;
  x
}

fn toggle(mut x: bool) -> bool {
  x = !x;
  x
}

fn f_decrease(mut x: f32) -> f32 {
  x += 0.01;
  x
}

fn main() {
  let x = 1;
  let y = true;
  let z = 1.1;

  let x1 = increase(x);
  let y1 = toggle(y);
  let z1 = f_decrease(z);

  println!("Origin Number -> {}", x); // pass "copy types" variable to func does not trigger a "move semantics" -> still accessable
  println!("Origin Boolean -> {}", y);
  println!("Origin Float -> {}", z);

  println!("Number -> {}", x1);
  println!("Boolean -> {}", y1);
  println!("Float -> {}", z1);
}
