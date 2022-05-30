use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("Guess the number!");
  
  let actual = rand::thread_rng().gen_range(1..101);
  
  loop {
    let guess = match get_guess() {
      Ok(num) => num,
      Err(_) => continue,
    };

    match guess.cmp(&actual) {
      Ordering::Less => println!("Too small"),
      Ordering::Equal => {
        println!("You got it!");
        break;
      },
      Ordering::Greater => println!("Too big"),
    }
  }
}


fn get_guess() -> Result<u32, std::num::ParseIntError> {
  println!("Please input your guess.");

  let mut raw_guess = String::new();
  io::stdin()
    .read_line(&mut raw_guess)
    .expect("Failed to read line");

  let guess: u32 = match raw_guess.trim().parse() {
    Ok(num) => num,
    Err(e) => return Err(e),
  };
  println!("You guessed: {}", guess);

  return Ok(guess);
}
