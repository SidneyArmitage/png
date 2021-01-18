

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_signature_true() {
      assert_eq!(is_signature([137, 80, 78, 71, 13, 10, 26, 10]), true);
  }

  #[test]
  fn test_signature_false() {
      assert_eq!(is_signature([0; 8]), false);
  }
}

pub fn is_signature(data: [u8; 8]) -> bool {
  data[0] == 137 &&
  data[1] == 80 &&
  data[2] == 78 &&
  data[3] == 71 &&
  data[4] == 13 &&
  data[5] == 10 &&
  data[6] == 26 &&
  data[7] == 10
}