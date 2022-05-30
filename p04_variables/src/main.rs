fn main() {
  // Scalars
  let x = 5;
  println!("The value is {}", x);
  let x = 6;
  println!("The value is {}", x);

  let c: char = 'h';
  println!("The character is {}", c);
  
  // Tuples
  let tup: (i32, f32, u8) = (3100, 1.52, 4);
  let (x0, x1, x2) = tup;
  println!("Tuple! ({}, {}, {})", tup.0, tup.1, tup.2);
  println!("Or with individual values! ({}, {}, {})", x0, x1, x2);
}
