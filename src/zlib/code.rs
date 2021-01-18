pub struct Fixed_Code {
  offsets: [u8; 512],
  values: [u16; 512],
}

impl Fixed_Code {
  pub fn new() -> Fixed_Code {
    let mut offsets = [0; 512];
    let mut values = [0u16; 512];
    let mut count = 0;
    for i in 0b000_0000..=0b001_0111 {
      let index = i << 2;
      values[index] = 143 + i as u16;
      values[index + 1] = 143 + i as u16;
      values[index + 2] = 143 + i as u16;
      values[index + 3] = 143 + i as u16;
      offsets[index] = 7;
      offsets[index + 1] = 7;
      offsets[index + 2] = 7;
      offsets[index + 3] = 7;
      count += 4;
    }
    for i in 0b0011_0000..=0b1011_1111 {
      let index = i << 1;
      values[index] = i as u16;
      values[index + 1] = i as u16;
      offsets[index] = 8;
      offsets[index + 1] = 8;
      count += 2;
    }
    for i in 0b1100_0000..=0b1100_0111 {
      let index = i << 1;
      values[index] = 280 + i as u16;
      values[index + 1] = 280 + i as u16;
      offsets[index] = 8;
      offsets[index + 1] = 8;
      count += 2;
    }
    for i in 0b1_1001_0000..=0b1_1111_1111 {
      values[i] = 144 + i as u16;
      offsets[i] = 9;
      count += 1;
    }
    Fixed_Code {
      offsets: offsets,
      values: values,
    }
  }
  // test for skip byte (starts on bit 7)
  pub fn process(&self, buf: &[u8], byte: usize, bit: u8) -> (u16, usize, u8) {
    let inverse = (9 - (bit as i8)) % 8;
    let mut cur = buf[byte] as u16 >> bit;
    if inverse != 0 {
      cur += (buf[byte +((bit as usize + 9) / 8)] as u16) << inverse;
    }
    cur = cur.reverse_bits();
    (
      self.values[cur as usize],
      byte +(((bit + self.offsets[cur as usize])as usize) / 8),
      (bit + self.offsets[cur as usize]) % 8,
    )
  }
}

// byte l-r pick r-l display l-r
pub fn fixed_code(buf: &[u8], byte: usize, bit: u8) -> (u16, usize, u8) {
  let inverse = (8 - (bit as i8)) % 8;
  let mut cur = buf[byte] >> bit;
  if inverse != 0 {
    cur += buf[byte + ((bit as usize + 8) / 8)] << inverse;
  }
  cur = cur.reverse_bits();
  if cur <= 0b0010_1111u8 {
    println!("256-279, {:b}", cur);
    return (
      (cur >> 1) as u16 + 256,
      byte + ((bit as usize + 7) / 8),
      (bit + 7) % 8,
    );
  } else if cur <= 0b1011_1111u8 {
    println!("0-143, {:b}", cur);
    return (cur as u16 - 0b0011_0000u16, byte + 1, bit);
  } else if cur <= 0b11000111u8 {
    println!("280-287, {:b}", cur);
    return (cur as u16 - 0b11000000u16 + 280, byte + 1, bit);
  } else {
    println!("144-255, {:b}", cur);
    let new_inverse = (9 - ((bit) as i8)) % 8;
    let new_cur = (cur as u16) << 1;
    println!("{}", new_inverse);
    println!("{:b}", buf[byte + ((bit as usize + 9) / 8)]);
    println!("{:b}", buf[byte + ((bit as usize + 9) / 8)] >> (8 - new_inverse));
    println!("");
    println!("{:b}", new_cur);
    println!("{:b}", buf[byte + 1]);
    println!("{:b}", buf[byte + 1] >> (bit));
    let over = match (bit + 9) % 8 {
      0 => ((buf[byte + 1]) >> bit) as u16 + new_cur,
      _ => (((buf[byte + ((bit as usize + 9) / 8)]) >> (8 - new_inverse)) as u16 + new_cur),
    };
    println!("{:b} {:b}", new_cur, over);
    return (
      over - 0b1_1001_0000 + 144,
      byte + ((bit as usize + 9) / 8),
      (bit + 9) % 8,
    );
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_skip_edge_case() {
    let input = [0b1000_0000u8, 0b1111_1111u8, 0b0000_0000u8];
    {
      let (out, byte, bit) = fixed_code(&input, 0, 7);
      assert_eq!(out, 255);
      assert_eq!(byte, 2);
      assert_eq!(bit, 0);
    }
  }
  #[test]
  fn test_fixed_code_std_success() {
    let input = [0x73u8, 0x49u8, 0x4du8, 0xcb, 0x49, 0x2c, 0x49, 0x55, 0, 0x11, 0];
    {
      let (out, byte, bit) = fixed_code(&input, 0, 3);
      assert_eq!(out, 0x44);
      assert_eq!(byte, 1);
      assert_eq!(bit, 3);
    }
    {
      let (out, byte, bit) = fixed_code(&input, 1, 3);
      assert_eq!(out, 0x65);
      assert_eq!(byte, 2);
      assert_eq!(bit, 3);
    }
  }
  #[test]
  fn test_fixed_code_max_success() {
    let input = [0b1111_0100u8, 0b1111_1110u8, 0b1111_1111u8, 0b1110_0011u8];
    {
      let (out, byte, bit) = fixed_code(&input, 0, 0);
      assert_eq!(out, 279);
      assert_eq!(byte, 0);
      assert_eq!(bit, 7);
    }
    {
      let (out, byte, bit) = fixed_code(&input, 0, 7);
      assert_eq!(out, 143);
      assert_eq!(byte, 1);
      assert_eq!(bit, 7);
    }
    {
      let (out, byte, bit) = fixed_code(&input, 1, 7);
      assert_eq!(out, 255);
      assert_eq!(byte, 3);
      assert_eq!(bit, 0);
    }
    {
      let (out, byte, bit) = fixed_code(&input, 3, 0);
      assert_eq!(out, 287);
      assert_eq!(byte, 4);
      assert_eq!(bit, 0);
    }
  }
  #[test]
  fn test_fixed_code_min_success() {
    let input = [0b0001_0011u8, 0b0001_1000u8, 0b0000_0000u8, 0b0000_0011u8];
    {
      let (out, byte, bit) = fixed_code(&input, 0, 0);
      assert_eq!(out, 144);
      assert_eq!(byte, 1);
      assert_eq!(bit, 1);
    }
    {
      let (out, byte, bit) = fixed_code(&input, 1, 1);
      assert_eq!(out, 0);
      assert_eq!(byte, 2);
      assert_eq!(bit, 1);
    }
    {
      let (out, byte, bit) = fixed_code(&input, 2, 1);
      assert_eq!(out, 256u16);
      assert_eq!(byte, 3);
      assert_eq!(bit, 0);
    }
    {
      let (out, byte, bit) = fixed_code(&input, 3, 0);
      assert_eq!(out, 0b100011000u16);
      assert_eq!(byte, 4);
      assert_eq!(bit, 0);
    }
  }


  #[test]
  fn test_constructor() {
    Fixed_Code::new();
  }
}
