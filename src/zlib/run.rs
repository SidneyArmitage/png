use super::code;
use super::retrieve::retrieve;

pub enum State {
  HLIT,
  HDIST,
  FINISH,
}

/// returns is end
pub fn run(buf: &[u8], byte: usize, bit: u8, get_code: &code::Fixed_Code) -> (State, usize, u8, u16) {

  let mut offset = bit;
  let mut index = byte;
  let (out, mut new_index, mut new_offset) = get_code.process(buf, index, offset);
  index = new_index;
  offset = new_offset;
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
        let (distance, new_index, new_offset) = retrieve(buf, index, offset, 1);
        index = new_index;
        offset = new_offset;
        distance + 11
      },
      269..=272 => {
        let (distance, new_index, new_offset) = retrieve(buf, index, offset, 2);
        index = new_index;
        offset = new_offset;
        distance + 19
      },
      273..=276 => {
        let (distance, new_index, new_offset) = retrieve(buf, index, offset, 3);
        index = new_index;
        offset = new_offset;
        distance + 35
      },
      277..=280 => {
        let (distance, new_index, new_offset) = retrieve(buf, index, offset, 4);
        index = new_index;
        offset = new_offset;
        distance + 67
      },
      281..=284 => {
        let (distance, new_index, new_offset) = retrieve(buf, index, offset, 5);
        index = new_index;
        offset = new_offset;
        distance + 131
      },
      285 => {
        258
      },
      _ => {
        panic!("invalid length code");
      }
    };
    // length
    return (State::HDIST, index, offset, distance);
  }

}

// pub fn distance(buf: &[u8], byte: usize, bit: u8) -> u8 {
//   let inverse = (8 - (bit as i8)) % 8;
//   let mut value = buf[byte] >> bit;
//   if inverse != 0 {
//     value += buf[byte + ((bit as usize + 8) / 8)] << inverse;
//   };
//   let extra = (value - 4) / 2 + 1;

// }


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_run_static_success() {
    let fixed = code::Fixed_Code::new();
    let input = [0x73u8, 0x48u8, 0x4du8, 0xcb, 0x49, 0x2c, 0x49, 0x55, 0, 0x11, 0];
    {
      let (state, index, bit, value) = run(&input, 0, 3, &fixed);
      assert_eq!(index, 1);
      assert_eq!(bit, 3);
      assert_eq!(value, 46); // raw
    }

    // out Defla
  }

}