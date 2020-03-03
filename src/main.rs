//#[macro_use] extern crate json;

mod request;

use std::env;
use self::request::{
    doinai_request,
    glive_request
};

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() > 1 {
      if &args[1] == "doinai" {
         let mut page: &str = "1";
         if args.len() > 2 {
            page = &args[2];
         }
         println!("{}", doinai_request(page));
      } else if &args[1] == "glive" {
        println!("{}", glive_request());
      }
  }
}
