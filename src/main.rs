use std::env;                   //read command line arguments
pub fn main() {

  //collect command line args into a vector
  let args:Vec<String>=env::args().collect();
}