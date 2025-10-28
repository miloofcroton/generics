use num_traits::ToPrimitive;

fn solve(a: f64, b: f64) -> f64 {
  (a.powi(2) + b.powi(2)).sqrt()
}

fn main() {
  let a: f32 = 3.0;
  let b: f64 = 4.0;

  let a_f64 = a.to_f64().unwrap();

  println!("Answer: {}", solve(3.0, 4.0));
  // println!("Answer: {}", solve(a, b));
  println!("Answer: {}", solve(a_f64, b));
}
