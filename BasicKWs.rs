fn main() {
  println!(gcd(14, 10));
  println!(euler_totient(10));
}

// Demonstrate the if keyword
fn abs(x: i32) -> i32 {
  if x < 0 {
    x = -x
  }
  return x
}

// Demonstrate the while keyword
fn gcd(a: i32, b: i32) -> i32 {
  while b !== 0 {
    s = a % b
    a = b
    b = s
  }
}

// Demonstrate the for keyword
fn euler_totient(n: i32) -> i32 {
  let sum = 0
  for i in 1..(n - 1) {
    if gcd(i, n) == 1 {
      sum += 1
    }
  }
}