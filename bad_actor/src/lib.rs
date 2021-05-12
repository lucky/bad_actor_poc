extern crate proc_macro;
use proc_macro::TokenStream;
use std::fs::File;
use std::io::prelude::*;
use std::io;

fn read_aws_credentials() -> Result<String, io::Error> {
    let home = std::env::var("HOME").unwrap();
    let mut file = File::open(home + "/.aws/credentials")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    match read_aws_credentials() {
        Err(e) => { println!("Error reading credentials: {}", e)}
        Ok(creds) => println!("Your aws credentials are: {}", creds)
    }

    "fn answer() -> u32 { 42 }".parse().unwrap()
}