use num_traits::{ToPrimitive, Float};

fn solve<T: Float, U: Float>(a: T, b: U) -> f64 {
  let a_f64 = a.to_f64().unwrap();
  let b_f64 = b.to_f64().unwrap();

  return (a_f64.powi(2) + b_f64.powi(2)).sqrt();
}

fn solve_v2<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
  let a_f64 = a.to_f64().unwrap();
  let b_f64 = b.to_f64().unwrap();

  return (a_f64.powi(2) + b_f64.powi(2)).sqrt();
}

fn main() {
  let a: f32 = 3.0;
  let b: f32 = 4.0;
  let c: i32 = 4;

  // let a_f64 = a.to_f64().unwrap();

  println!("Answer: {}", solve(3.0, 4.0));
  // println!("Answer: {}", solve(a_f64, b));
  println!("Answer: {}", solve::<f32, f32>(a, b));
  println!("Answer: {}", solve(a, b));
  println!("Answer: {}", solve_v2(a, c));
}
