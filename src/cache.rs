#![allow(dead_code)]
use std::fs::File;                // file handling
use std::io::{BufReader,BufRead};// buffered reading of trace file

use crate::{parse_address,extract_address};
use crate::policy::{ReplacementPolicy,LRU,FIFO};

//represent a single cache line
//stores only metadata, not actual memory content
// we are storing address  too because this will help us 
//extending our design to multilevel cache
// it is making basic cache operation fucntion easier to write and use 
//as we can parse address and can get tag adn set index field for those functiones
// instead of passing as arguments.

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
  policy:Box<dyn ReplacementPolicy>,
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
  FIFO, // NEW policy added
}

/*
implementation block for line
responsible for creating  an empty cache line
with contain_block set to false, tag and address as zero , as default placeholder values.
*/
impl Line{
  fn new()->Self{
    Self{
        contain_block:false,
        tag:0,
        address:0,
    }
  }
}

/*
implementation block for creating a cache set, 
each line within a set is initialise using Line::new() constructior
each set is associated with a replacement policy instance
selection of policy is performed using PolicyType enumeration
LRU policy is used 
*/
impl Set{
  fn new(e:usize,policy_type:PolicyType)->Self{
    let mut lines:Vec<Line>=Vec::new();
    for _ in 0..e{
      lines.push(Line::new());
    }
    // initilise policy based on type
    let policy:Box<dyn ReplacementPolicy>=match policy_type{
      PolicyType::LRU=>Box::new(LRU::new(e)),
      PolicyType::FIFO => Box::new(FIFO::new(e)),
    };
    Self{lines,policy}
  }
}

/*
cache implementation block for creating a cache form user specified parameters.
set is dynamically allocated to stick to specifications.
this also maintain counters for cache statistics suh as hits, misses and evicts
also stores number of set index bits to caclulate total number of sets in cache 
*/
impl Cache{
  fn new(s:usize, e:usize,b:usize,policy_type:PolicyType)->Self{
    let mut sets:Vec<Set>=Vec::new();
    let total_sets:usize=1<<s;

    for _ in 0..total_sets{
      sets.push(Set::new(e,policy_type));
    }

    Self{sets,s,b,hits:0,misses:0,evicts:0,}
  }
}

/*
implementation block containig basic cache function needed to operate on cache
*/
impl Cache{

  /*  
  searches address if present in a set or not 
  if present increment the cache hit count and return enum HIT to indicate successful search.
  if address not found, then its a miss, so increment cache miss count and return 
  SearchResult::MISS to indicate a miss.
  */
  fn  search(&mut self,address:u64)->SearchResult{
    //use set index to determine set
    //search in lines of that set
    
    let (tag,set_index)=parse_address(address,self.s,self.b);
    let set:&mut Set=&mut self.sets[set_index];

    // iterate over all lines present in a set 
    for (line_index,line) in set.lines.iter_mut().enumerate(){
      // found match for extracted tag form address, 
     // if match found its a hit
     // record this cache access
      if line.contain_block && line.tag==tag{
        // record cache access on hit
        // use lru implementation of policy that we have provided
        
        set.policy.record_cache_access(line_index);
        self.hits+=1;// this again ensure in  design we dont have to write again and again this same line if search return a HIT
        return SearchResult::HIT;
      }
    }
    self.misses+=1;// this again ensure in design we dont have to write again and again this same line if search return a MISS
    SearchResult::MISS 
  }

  /*
  this fucntion handles core cache operation based on instruction type
  load, store or modified(load+store)
  */
  #[allow(unused_variables)]
  fn  operate(&mut self,address:u64, start_char:char){
    
    // for load or store instruction
    if start_char=='L'|| start_char=='S'{
      // if address found its a hit
      let result:SearchResult=self.search(address);
    } 
  }
}

// this function is called from simulator entry point main
pub fn process_trace_file(s:usize,e:usize,b:usize,trace_file:&str,policy_type:PolicyType)->(u64,u64,u64){
  let file=match File::open(trace_file){
    Ok(file)=>file,
    Err(err)=>{
      println!("failed to open the file: {}",err);
      std::process::exit(1);
    }  
  };
  // this function initilise a cache object form received arguments
  let mut cache:Cache=Cache::new(s,e,b,policy_type);

  let reader=BufReader::new(file);

  // read trace file line by line
  for line in reader.lines(){
    let address_str=match line{
      Ok(v)=>v,
      Err(_)=>continue,
    };

    let trimmed_address_str=address_str.trim();

    // if trace line empty , continue for next line
    if trimmed_address_str.is_empty(){
      continue;
    }

    let start_char=trimmed_address_str.chars().next().unwrap();

    // if trimmed trace line does not starts with instruction M|L|S continue to next line 
    if ! matches!(start_char, 'M' | 'L' | 'S'){
      continue;
    }

    // extract the memory address safely upon which we will operate our caceh form this trace line
    let address:u64=match extract_address(&trimmed_address_str){
      Some(addr)=>addr,
      None=>continue,
    };

    //perform cache operation on this address based on instruction type
    cache.operate(address,start_char);
  }

  // return final cache statistics such as hits , misses and evicts to the caller
  (cache.hits,cache.misses,cache.evicts)
}
