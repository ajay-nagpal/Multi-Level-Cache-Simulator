// create a gneric  interface for replacement policies
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