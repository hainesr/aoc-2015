//
// Advent of Code 2015
//
// Robert Haines
//
// Public Domain
//

fn main() {
  let key = "yzbqklnj";

  println!("Part 1: {}", find_suffix(key, 200_000));
}

fn find_suffix(key: &str, start: u32) -> u32 {
  let mut i = start;

  loop {
    let digest = md5::compute(key.to_string() + &i.to_string());

    if &format!("{:x}", digest)[0..5] == "00000" {
      return i;
    }

    i += 1;
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_suffix() {
      assert_eq!(609_043, find_suffix("abcdef", 600_000));
    }
}
