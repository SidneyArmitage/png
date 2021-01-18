use super::super::*;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_success() {
    let mut options = Options::new();
    options.colour_type = 2;
    assert_eq!(process(&[0x00, 0x30, 0x00, 0x00, 0x00, 0x9C, 0x9E, 0x9C, 0x5A, 0x5D, 0x00, 0x40, 0x5A, 0xDE, 0xDF, 0xDE, 0x39, 0x3C, 0x39, 0x18, 0x1C, 0x18, 0xBD, 0xBE, 0xBD, 0x7B, 0x7D, 0x7B, 0x00, 0x50, 0xFF, 0xFF, 0xFF, 
      ], 27, &mut options), Ok(()));

  }
}
// must be called on colour 3 2, 6 optional
pub fn process(buffer: &[u8], length: u32, options: &mut Options) -> Result<(), ()> {
  if (length % 3) != 0 {
    println!("invalid number of palette entries");
    return Err(());
  }
  if options.colour_type == 0 || options.colour_type == 4 {
    println!("due to colour type PLTE is not expected");
    return Err(());
  }
  for i in 0..(length/3) {
    options.plte.push([buffer[(i * 3) as usize], buffer[(i * 3 + 1) as usize], buffer[(i * 3 + 2) as usize]]);
  }
  Ok(())
}