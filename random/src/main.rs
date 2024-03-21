
fn main() {
  println!("{}", compute_irrational(100));
}
fn compute_irrational(n: u32) -> String {
  let mut num: String = "0.00".to_string();
  let mut ones_pos: usize;
  for i in 0..n {
    ones_pos = num.find('.').unwrap() - 1;
    num = string_add_at(num, ones_pos, 1);
    num = string_flatten(num);
    println!("{} gives {}", i + 1, num);
  }
  return num;
}

fn string_add_at(num: String, pos: usize, add: i8) -> String {
  let mut num_cpy: String = num.clone();
  let (first, rest) = num_cpy.split_at_mut(pos);
  let (ones, second) = rest.split_at_mut(1);
  // println!("in string_add_at: {}, {}, {}", first, ones, second); // nice debug
  let ones_next: String = char_add(ones, add);
  return first.to_owned() + &ones_next + second;
}

// there's STILL a better way to do this
fn char_add(str: &str, add: i8) -> String {
  if add < 0 {
    let diff: u8 = -add as u8;
    return String::from((str.chars().next().unwrap() as u8 - diff) as char);
  }
  else {
    return String::from((str.chars().next().unwrap() as u8 + add as u8) as char);
  }
}

fn string_verify_flat(num: String) -> bool {
  // println!("{}", '.' as u8); outputs 46, so '.' < '3' is true
  return num.chars().all(|x: char| x < '3');
}

fn string_flatten(num: String) -> String {
  if string_verify_flat(num.clone()) {
    return num;
  }
  //println!("In string_flatten beginning: {}", num);
  // remove decimal
  let mut dec_pos: usize = num.find('.').unwrap();
  let mut num_spl = num.clone();
  let (num1, rest) = num_spl.split_at_mut(dec_pos);
  let (_dec, num2) = rest.split_at_mut(1);
  // println!("{}, {}, {}", first, ones, second); // nice debug
  let mut new_num = num1.to_string() + &num2;
  // since we failed to verify, some character is greater than '2' - make everything smaller
  let mut pos: usize = new_num.find(|c: char| c > '2').unwrap();
  // deal with cases where we need to add zeroes near the ends
  if pos >= new_num.len() - 2 {
    new_num = new_num + "00";
  }
  if pos == 0 {
    new_num = "0".to_string() + &new_num;
    pos += 1;
    dec_pos += 1
  }
  // perform the actual changes
  new_num = string_add_at(new_num, pos - 1, 1);
  new_num = string_add_at(new_num, pos, -3);
  new_num = string_add_at(new_num, pos + 2, 2);
  //println!("In string_flatten ending: {}", new_num);
  // put the decimal back in its place
  let (new1, new2) = new_num.split_at_mut(dec_pos);
  return string_flatten(new1.to_string() + "." + &new2);
}

fn string_complete(num: String) -> String {
  let mut new_num: String = num.clone();

  return num;
}