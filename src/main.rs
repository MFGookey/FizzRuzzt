use crate::game::fizzbuzz::FizzBuzz;
mod game;

fn main() {
    // For 1 to 100
    // if x is evenly divisible by 15, print FizzBuzz
    // if x is evenly divisible by 5, print Buzz
    // if x is evenly divisible by 3, print Fizz
    // otherwise print x

    /*for i in 1..=30 {
      println!("{}", fizzbuzzchain(i));
    }

    for i in 31..=60 {
      println!("{}", fizzbuzzsequenced(i));
    }

    for i in 61..=100 {
      println!("{}", fizzbuzzfunctional(i))
    }*/

    /*for i in 1..=100 {
      println!("{}", FizzBuzz::from(i));
    }*/

    for f in (-100..=100).into_iter().map(|x| FizzBuzz::from(x)) {
        println!("{}", f);
    }
}
/*
fn fizzbuzzchain(i : u32) -> String {
  if i % 5 == 0 && i % 3 == 0 {
    "FizzBuzz".to_string()
  }
  else if i % 5 == 0 {
    "Buzz".to_string()
  }
  else if i % 3 == 0 {
    "Fizz".to_string()
  }
  else {
    i.to_string()
  }
}


fn fizzbuzzsequenced(i : u32) -> String {
  let mut result: String  = String::new();

  if i % 3 == 0 {
    result += "Fizz";
  }

  if i % 5 == 0 {
    result += "Buzz";
  }


  if result.is_empty() {
    result = i.to_string();
  }

  result
}

fn fizzbuzzfunctional(i : u32) -> String {
  match (i % 3, i % 5) {
    (0, 0) => "FizzBuzz".to_string(),
    (0, _) => "Fizz".to_string(),
    (_, 0) => "Buzz".to_string(),
    _ => i.to_string()
  }
}*/
