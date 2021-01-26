// Length = distance copied
// distance how far back to look
use super::run;
use super::code;


// literals lose 0x30 and get reversed

/*
do
  read block header from input stream.
  if stored with no compression
    skip any remaining bits in current partially
        processed byte
    read LEN and NLEN (see next section)
    copy LEN bytes of data to output
  otherwise
    if compressed with dynamic Huffman codes
        read representation of code trees (see
          subsection below)
    loop (until end of block code recognized)
        decode literal/length value from input stream
        if value < 256
          copy value (literal byte) to output stream
        otherwise
          if value = end of block (256)
              break from loop
          otherwise (value = 257..285)
              decode distance from input stream

              move backwards distance bytes in the output
              stream, and copy length bytes from this
              position to the output stream.
    end loop
while not last block
*/

fn process(buf: &[u8], fixed_code: &code::Fixed_Code) -> (Vec<u8>, usize, bool) {
  let is_final = buf[0] & 0b10000000u8 != 0;
  match buf[0] & 0b01100000u8 {
    0 => {
      // no compression
      let len = u16::from_be_bytes([buf[1], buf[2]]);
      let n_len = u16::from_be_bytes([buf[3], buf[4]]);
      if len & n_len != 0 {
        panic!("complements don`t match");
      }
      return ((buf[5..(len+5) as usize].to_vec()), len as usize, is_final);
    },
    0b00100000u8 => {
      // fixed huffman codes
      let mut literals: Vec<u8> = vec!();
      let mut out: Vec<u8> = vec!();
      let mut state = run::State::HLIT;
      let mut byte = 0;
      let mut bit = 0;
      loop {
        let (new_state, new_byte, new_bit, value) = run::run(buf, byte, bit, fixed_code);
        state = new_state;
        byte = new_byte;
        bit = new_bit;
        match state {
          run::State::HLIT => {
            literals.push(value as u8)
          },
          run::State::HDIST => {
            // read 5 bits
            let length = value;
          },
          run::State::FINISH => {
            break
          },
        };
      }
    },
    0b01000000u8 => {
      // dynamic huffman codes
      // read huffman tree
    },
    _ => {
      panic!("unexpected compression method");
    }
  }
  (vec!(), 0,is_final)
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_no_compression_success() {
    let input = [0b10000000u8,0, 5, 0b11111111, 0b11111010, 0, 1, 2, 3, 4];
    let fixed = code::Fixed_Code::new();
    let (buf, size, end) = process(&input, &fixed);
    assert_eq!(end, true);
    println!("block len: {}", size);
    assert_eq!(size, 5);
    println!("block: {}", buf[0]);
    assert_eq!(buf[0], input[5]);
    assert_eq!(buf[1], input[6]);
    assert_eq!(buf[2], input[7]);
    assert_eq!(buf[3], input[8]);
    assert_eq!(buf[4], input[9]);
  }

  #[test]
  #[should_panic]
  fn test_no_compression_complement_fail() {
    let input = [0b10000000u8,0, 5, 0b11111111, 0b11111110, 0, 1, 2, 3, 4];
    let fixed = code::Fixed_Code::new();
    process(&input, &fixed);
  }


  fn test_compression() {
    // assert_eq!(process(in_GNU), out_GNU);
  }

  
  
  

}