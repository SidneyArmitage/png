mod block;

pub mod code;
pub mod run;
pub mod retrieve;

fn header() {
  // CMF
  //  CM 0-3 expect 8
  //  CINFO 4-7 log 2 of window size
  // FLG
  //  FCHECK 0-4
  //  FDICT 5 (Check for dict)
  //  FLEVEL 6-7 (not needed, only for compression)
}

fn footer() {
  // adler32 checksum
}
