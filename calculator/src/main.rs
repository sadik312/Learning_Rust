use std::env::{args, Args};

fn main() {
    let mut args: Args = args();
    
    let first: String = args.nth(1).unwrap();
}