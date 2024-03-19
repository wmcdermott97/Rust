fn main() {
  println!("{}", gcd(14, 10));
  println!("{}", euler_totient(127));
}

// Demonstrate the if keyword
fn abs(x: i32) -> i32 {
  if x < 0 {
    return -x;
  }
  return x;
}

// Demonstrate the while keyword
fn gcd(a: i32, b: i32) -> i32 {
  let mut x = abs(a);
  let mut y = abs(b);
  while y != 0 {
    (x, y) = (y, x % y)
  }
  return x;
}

// Demonstrate the for keyword
fn euler_totient(n: i32) -> i32 {
  let mut sum = 0;
  for i in 1..n {
    if gcd(i, n) == 1 {
      sum += 1;
    }
  }
  return sum;
}