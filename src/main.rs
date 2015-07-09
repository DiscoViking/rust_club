extern crate rand;
mod sleeping_barbers;

use std::env;
use sleeping_barbers::sleeping_barbers;

fn main() {
   let mut args = env::args();

   // Throw away the first argument since it contains the program name.
   args.next();

   // Run any others.
   for a in args {
       run_problem(a);
   }
}

fn run_problem(name: String) {
    println!("=== Running {:?} ===", name);
    match name.as_ref() {
        "sleeping_barbers" => sleeping_barbers(),
        _ => println!("{:?} is not a valid problem.", name),
    };
    println!("");
}
