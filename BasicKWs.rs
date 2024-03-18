fn main() {
  println!("{}", gcd(14, 10));
  println!("{}", euler_totient(10));
}

// Demonstrate the if keyword
fn abs(mut x: i32) -> i32 {
  if x < 0 {
    x = -x;
  }
  return x;
}

// Demonstrate the while keyword
fn gcd(mut a: i32, mut b: i32) -> i32 {
  a = abs(a);
  b = abs(b);
  while b != 0 {
    let s = a % b;
    a = b;
    b = s;
  }
  return a;
}

// Demonstrate the for keyword
fn euler_totient(n: i32) -> i32 {
  let mut sum = 0;
  for i in 1..(n - 1) {
    if gcd(i, n) == 1 {
      sum += 1;
    }
  }
  return sum;
}