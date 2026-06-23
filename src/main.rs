use std::env;                   //read command line arguments
use multi_level_cache_simulator::{print_err_msg, parse_args};// import utility fucntion used here 
use multi_level_cache_simulator::cache::{process_trace_file,PolicyType};// import core simulation components

#[allow(unused_variables)]
pub fn main() {

  //collect command line args into a vector
  let args:Vec<String>=env::args().collect();
  // use parse_args utility function to extrace the values of 
  //s: number of set index bits
  //e: number of lines in a set
  //b: number of block offset bits
  // trace_file: path to the file containing memory access
  let (s,e,b,trace_file):(usize,usize,usize,String)=match parse_args(&args){
    //successful parse, extract values
    Some(v)=>v,

    // anything invalid, return to prevent undefined behaviour
    None=>{
      print_err_msg();
      return;
    }
  };

  if s==0 || b==0||e==0|| b>=64||(s+b)>=64 {
    println!("invalid flag(s|b|E) value!");
    return;
  }
  //process the trace file get the cache staticstics
  // policy type if passed explicitly
  // this design allow future extension
  let (hits,misses,evictions)=process_trace_file(s,e,b,trace_file.as_str(),PolicyType::LRU);
}