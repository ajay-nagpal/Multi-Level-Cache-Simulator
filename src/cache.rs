#![allow(dead_code)]
use std::fs::File;                // file handling
use crate::policy::ReplacementPoicy;

//represent a single cache line
//stores only metadata, not actual memory content
// we are storing address  too because this will help us 
//extending our design to multilevel cache
// it is making basic cache operation fucntion easier to write and use 
//as we can parse address and can get tag adn set index field for those functiones, instead of passing as arguments.

struct Line{
  contain_block:bool,// indicate whether a line is logiacally occupied
  tag:u64,            // to identify stored  block in memory
  address:u64,
}

/*
represent a cache set
each set contain multiple lines
we have implemented ReplacementPoicy per set basis 
*/

struct Set{
  lines:Vec<Line>,
  // we will create a policy obj per set basis
  policy:Box<dyn ReplacementPoicy>,
}

/*
represent the entire cache structure
stores various cache parameters and statistics counters
*/
struct Cache{
  sets:Vec<Set>,//collection of cache set
  s:usize,     // number of set index bits
  b:usize,    // number of block offset bits

  // members to store cache hit,mis and evict status
  hits:u64,
  misses:u64,
  evicts:u64,
}

#[derive(PartialEq,Debug,Eq)] // attribute to allow enum values to be comared using == or !=
enum SearchResult{
  // enumeration to represent cache search result
  // HIT indicate address block is in cache , MISS indicate block is not in cache need to insert if space available
  HIT,
  MISS,
}

// enumeration representing policy type with copyable and comparable behaviour 
// currently only LRU is implemetned
// but design allowed for future extension

#[derive(Copy,Clone,PartialEq)] 
pub enum PolicyType{
  LRU,
}

#[allow(unused_variables)]
// this function is called from simulator entry point main
pub fn process_trace_file(s:usize,e:usize,b:usize,trace_file:&str,policy_type:PolicyType)->(u64,u64,u64){
  let file=match File::open(trace_file){
    Ok(file)=>file,
    Err(err)=>{
      println!("failed to open the file: {}",err);
      std::process::exit(1);
    }  
  };
  todo!()
}
