use getopts::Options;              //Cli arguments parsing

// declare modules as public so that files can see it.
pub mod cache;
pub mod policy;

//to print help message when cli is invalid
pub fn  print_err_msg(){
  println!("invalid cli!");
  println!("required flags : -s <s> -E <E> -b <b> -t <tracefile>");
}

//extract a memory address from a trace line
// for any invalid trace line it returns None to ensure correctness
// otherwise it return Some(address)
fn  extract_address(line:&str)->Option<u64>{

  let parts:Vec<&str>=line.split_whitespace().collect();

  if parts.len()!=2{
    return None;
  }

  let adr_part:Vec<&str>=parts[1].split(',').collect();

  if adr_part.len()!=2{
    return None;
  }

  let adr_str:&str=adr_part[0];

  //converts hexadecimal string into a u64 safely by avoiding panic!
  //uses match to handle parsing error by returning None
  let address:u64=match u64::from_str_radix(adr_str,16){
    Ok(addr)=>addr,
    Err(_)=>return None,
  };
  
  Some(address)
}

//This function parse the command line args and extracts the cache parameters
// this design is expandable for allowing future extension in cli, like adding -p flag for policy
// we return None form this function in any invalid input case
pub fn parse_args(args:&Vec<String>)->Option<(usize,usize,usize,String)>{
  
  //strict enforce to take all 4 required flags 
  if args.len()!=9{
    print_err_msg();
    return None;
  }

  let mut opts=Options::new();

  //defining expected options
  opts.optopt("s","","","");
  opts.optopt("b","","","");
  opts.optopt("E","","","");
  opts.optopt("t","","","");

  // parse the arguments safely
  let matches:getopts::Matches=match opts.parse(&args[1..]){
    Ok(m)=>m,
    Err(_)=>{
      print_err_msg();
      return None;
    }
  };

  //reject unexpected arguments and return None if any invalid arg in cli.
  if !matches.free.is_empty(){
    print_err_msg();
    return None;
  }

  //retrive valusfor each required flag
  let s=matches.opt_str("s");
  let b=matches.opt_str("b");
  let e=matches.opt_str("E");
  let t=matches.opt_str("t");

  //ensure all required flags are present
  if s.is_none()||b.is_none()||e.is_none()||t.is_none(){
    print_err_msg();
    return None;
  }

  //parse numeric value safely, handle error by returning None
  let (s,e,b,trace_file):(usize,usize,usize,String)=match(
    s.unwrap().parse(),
    e.unwrap().parse(),
    b.unwrap().parse(),
    t.unwrap(),
  ){
    (Ok(sv),Ok(ev),Ok(bv),tv)=>(sv,ev,bv,tv),
    _ =>{
      print_err_msg();
      return None;
    }
  };

  //return Some(value) of validated cli flag values. 
  Some((s,e,b,trace_file))
}
