fn main() {
  println!("{}", mod_exp(5, 2, 23));
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

// writing some number theory
fn mod_exp(b: i32, x: i32, p: i32) -> i32 {
  let mut y = 1;
  let mut pow = b;
  let mut exp = x;
  while exp > 0 {
    if exp % 2 == 1 {
      y *= pow;
      y %= p;
    }
    pow *= pow;
    pow %= p;
    exp /= 2;
  }
  return y;
}