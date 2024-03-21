
fn main() {
  println!("{}", compute_irrational(3));
}
fn compute_irrational(n: u32) -> String {
  let mut num: String = "0.0".to_string();
  let mut ones_pos: usize;
  for _i in 0..n {
    ones_pos = num.find('.').unwrap() - 1;
    num = string_increment_at(num, ones_pos);
    num = string_simplify(num);
  }
  return num;
}

fn string_increment_at(num: String, pos: usize) -> String {
  let mut num_cpy: String = num.clone();
  let (first, rest) = num_cpy.split_at_mut(pos);
  let (ones, second) = rest.split_at_mut(1 as usize);
  println!("{}, {}, {}", first, ones, second); // nice debug
  let ones_next: String = char_increment(ones);
  return first.to_owned() + &ones_next + second;
}

// there's a better way to do this
fn char_increment(str: &str) -> String {
  return String::from((str.chars().next().unwrap() as u8 + 1) as char);
}

fn string_verify(num: String) -> bool {
  //   println!("{}", '.' as u8); outputs 46, so '.' < '3' is true
  return num.chars().all(|x: char| x < '3');
}

fn string_simplify(num: String) -> String {
  if string_verify(num.clone()) {
    return num;
  }
  
  return "no.".to_string();
}