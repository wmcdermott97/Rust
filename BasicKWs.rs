// Entry point for Rust
// (3, 5), (6, 7), (10, 11), (16, 17), (22, 23), (2, 29)
fn main() {
  square_prm()
}

// Demonstrate the if keyword
fn abs(x: i32) -> i32 {
  if x < 0 {
    return -x;
  }
  return x;
}

// Demonstrate the while keyword
fn gcd(a: u32, b: u32) -> u32 {
  let mut x = a;
  let mut y = b;
  while y != 0 {
    (x, y) = (y, x % y)
  }
  return x;
}

// Demonstrate the for keyword
fn euler_totient(n: u32) -> u32 {
  let mut sum = 0;
  for i in 1..n {
    if gcd(i, n) == 1 {
      sum += 1;
    }
  }
  return sum;
}

// Computes b^x modulo n
fn mod_exp(b: u32, x: u32, n: u32) -> u32 {
  let mut y = 1;
  let mut pow = b;
  let mut exp = x;
  while exp > 0 {
    if exp % 2 == 1 {
      y *= pow; y %= n;
    }
    pow *= pow; pow %= n;
    exp /= 2;
  }
  return y;
}

fn square_prm() {
  for p in [3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53] {
    for b in 2..(p - 1) {
      let val = mod_exp(b, p - 1, p * p);
      if val / p == 1 {
        println!("({}, {})", b, p)
      }
    }
  }
}