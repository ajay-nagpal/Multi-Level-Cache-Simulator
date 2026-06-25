use std::u64;

// create a generic  interface for replacement policies
// which will be implemented by policies

pub trait ReplacementPoicy{
  // struct implementing this trait will provide implementations for these

  //for updating policy metadatawhen a cache line is accessed
  fn record_cache_access(&mut self, line_index:usize);

  // to determine which cache line should be evicted when a set is full
  fn select_target(&mut self)->usize;

  fn reset_on_evict(&mut self, _index:usize){
    // default implementation
  }
}

//we will use LRU policy
// structure for least recently used replacement policy which 
//evict the line that has not been accessed for longest time
pub struct LRU{

  // to store the last access timestamp for each line
  line_last_used:Vec<u64>,

  //logical clock representing time counter which increments on each access
  time_counter:u64,
}

impl LRU{
  //a constructior for lru policy instance for a set with 'e'(number of line in a set) lines.
  pub fn new(e:usize)->Self{
    Self{
      line_last_used:vec![0;e],
      time_counter:0,
    }
  }
}

impl ReplacementPoicy for LRU{
  
  //for updating policy metadata when a cache line is accessed
  // or in short to record cache access 
  //done when insert and hit 
  fn record_cache_access(&mut self, line_index:usize){
    self.line_last_used[line_index]=self.time_counter;
    self.time_counter+=1;
  }

  //to determine which cache line should be evicted when a set is full
  // used in cache core functions
  // the hardcoded lru logic can be used here
  // this will return the selected target for eviction based on lru policy
  fn select_target(&mut self)->usize{
    let mut lru_min_time=u64::MAX;
    let mut target_index:usize=0;

    //search for least recently used line , return it's line index for eviction purpose
    for (index,&time) in self.line_last_used.iter().enumerate(){
      if time<lru_min_time{
        lru_min_time=time;
        target_index=index;
      }
    }
    target_index
  }
}