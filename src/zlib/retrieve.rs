pub fn retrieve(buf: &[u8], byte: usize, bit: u8, distance: u8) -> (u16, usize, u8) {
  let value = (buf[byte].reverse_bits() << bit) >> bit;
  if bit + distance > 8 {
    let (carry, new_byte, new_bit) = retrieve(buf, byte + 1, 0, distance - (8 - bit));
    return (
      ((value as u16) << (distance - (8 - bit)) as u16) + carry,
      new_byte,
      new_bit,
    );
  }
  (
    (value >> (8 - distance - bit)) as u16,
    (byte + ((bit + distance) / 8) as usize),
    (bit + distance) % 8,
  )
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_retrieve_single() {
    assert_eq!(retrieve(&[255], 0, 2, 3), (7, 0, 5));
  }

  #[test]
  fn test_overflow_single() {
    assert_eq!(retrieve(&[255, 255], 0, 2, 7), (127, 1, 1));
  }
  #[test]
  fn test_overflow_multi() {
    assert_eq!(retrieve(&[255, 255, 255, 255], 0, 7, 15), (32767, 2, 6));
  }

  #[test]
  fn test_endian() {
    assert_eq!(retrieve(&[0b1010_1010, 0b1010_1010], 0, 2, 7), (42, 1, 1));
  }
  #[test]
  fn test_endian_overflow_multi() {
    assert_eq!(retrieve(&[0b1010_1010, 0b1010_1010, 0b1010_1010, 0b1010_1010], 0, 7, 15), (21845, 2, 6));
    // 010_1010_1010_1010
  }
}