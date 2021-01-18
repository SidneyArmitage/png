
pub enum State {
  HLIT,
  HDIST,
  FINISH,
}

/// returns is end
pub fn run(buf: &[u8], byte: usize, bit: u8, code: fn(&[u8], usize, u8) -> (u16, usize, u8)) -> (State, usize, u8, u16) {

  let mut offset = bit;
  let mut index = byte;
  let (out, mut index, mut offset) = code(buf, index, offset);
  if out < 256 {
    // copy value to output
    println!("{:b}", out);
    return (State::HLIT, index, offset, out);
  } else if out == 256 {
    return (State::FINISH, index, offset, 0);
    // break
  } else {
    // decode distance
    let distance: u16 = match out {
      257..=264 => {
        3
      },
      265..=268 => {
        let distance = (11 + buf[index + ((offset as usize + 1) / 8)] & (0b10000000u8 >> (offset + 1) % 8)) as u16;
        offset += 1 % 8;
        index += (offset as usize + 1) / 8;
        distance
      },
      269..=272 => {
        let distance = 19 + (u16::from_be_bytes([buf[index], buf[index + 1]]) & (0b11000000_00000000u16 >> offset + 1)) as u16;
        offset += 2 % 8;
        index += (offset as usize + 2) / 8;
        distance
      },
      273..=276 => {
        let distance = 35 + (u16::from_be_bytes([buf[index], buf[index + 1]]) & (0b11100000_00000000u16 >> offset + 1)) as u16;
        offset += 3 % 8;
        index += (offset as usize + 3) / 8;
        distance
      },
      277..=280 => {
        let distance = 67 + (u16::from_be_bytes([buf[index], buf[index + 1]]) & (0b11110000_00000000u16 >> offset + 1)) as u16;
        offset += 4 % 8;
        index += (offset as usize + 4) / 8;
        distance
      },
      281..=284 => {
        let distance = 131 + (u16::from_be_bytes([buf[index], buf[index + 1]]) & (0b11111000_00000000u16 >> offset + 1)) as u16;
        offset += 5 % 8;
        index += (offset as usize + 5) / 8;
        distance
      },
      285 => {
        258
      },
      _ => {
        panic!("invalid distance code");
      }
    };
    // length
    return (State::HDIST, index, offset, distance);
  }

}



#[cfg(test)]
mod tests {
  use super::*;
  use super::super::code;

  #[test]
  fn test_run_static_success() {
    let input = [0x73u8, 0x48u8, 0x4du8, 0xcb, 0x49, 0x2c, 0x49, 0x55, 0, 0x11, 0];
    {
      let (state, index, bit, value) = run(&input, 0, 3, code::fixed_code);
      assert_eq!(index, 1);
      assert_eq!(bit, 3);
      assert_eq!(value, 46); // raw
    }

    // out Defla
  }

}