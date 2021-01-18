use super::super::*;


#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_success() {
    let mut options = Options::new();
    let out = process(&[0, 0, 0, 0x20, 0, 0, 0, 0x20, 0x10, 0x06, 0, 0, 0], 17, &mut options);
    assert_eq!(out, Ok(()));
    assert_eq!(options.bit_depth, 16);
    assert_eq!(options.colour_type, 6);
    assert_eq!(options.height,0x20);
    assert_eq!(options.interlace, false);
    assert_eq!(match options.state {
      State::IHDR => true,
      _ => false,
    }, true);
    assert_eq!(options.width, 0x20);
  }

  #[test]
  fn test_validator() {
    assert_eq!(colour_and_bit_is_valid(6, 16), true);
  }
}

fn colour_and_bit_is_valid(colour_type: u8, bit_depth: u8) -> bool {
  match colour_type {
    0 => match bit_depth {
      1 | 2 | 4 | 8 | 16 => true,
      _ => false,
    },
    2 => match bit_depth {
      8 | 16 => true,
      _ => false,
    },
    3 => match bit_depth {
      1 | 2 | 4 | 8 => true,
      _ => false,
    },
    4 => match bit_depth {
      8 | 16 => true,
      _ => false,
    },
    6 => match bit_depth {
      8 | 16 => true,
      _ => false,
    },
    _ => false,
  }
}

pub fn process(buffer: &[u8], length: u32, options: &mut Options) -> Result<(), ()>{
  if !colour_and_bit_is_valid(buffer[9], buffer[8]) {
    println!("Invalid colour and bit depth");
    return Err(());
  }
  // compression method u8
  if buffer[10] != 0 {
    println!("Compression method does not equal 0");
    return Err(());
  }
  // filter method u8
  if buffer[11] != 0 {
    println!("Filter method does not equal 0");
    return Err(());
  }
  // interlace method u8
  let interlace = buffer[12] == 0;
  if !interlace && buffer[12] != 0 {
    println!("Invalid interlace method");
    return Err(());
  }
  options.bit_depth = buffer[8];
  options.colour_type = buffer[9];
  options.width = u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
  println!("{:#?}", [buffer[0], buffer[1], buffer[2], buffer[3]]);
  println!("{}", options.width);
  options.height = u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
  options.state = State::IHDR;
  Ok(())
}