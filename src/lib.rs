use getopts::Options;              //Cli arguments parsing

// declare modules as public so that files can see it.
pub mod cache;
pub mod policy;

//to print help message when cli is invalid
pub fn  print_err_msg(){
  println!("invalid cli!");
  println!("required flags : -s <s> -E <E> -b <b> -t <tracefile>");
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
