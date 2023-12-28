use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::error::Error;
use std::io::stderr;
extern crate greprs;
use greprs::Config;


fn main() {
    let args: Vec<String>=env::args().collect();
    // let query=& args[1];
    // let filename=& args[2];
    // let (query,filename)=parse_config(& args);
    let config=Config::new(&args)
        .unwrap_or_else(|err|{
            writeln!((&mut stderr)(),"Problem parsing arguments: {}",err)
                .expect("Could not write to stderr");
            process::exit(1);
        });
    // println!("Searching for {}",config.query);
    // println!("In file {}",config.filename);
    // println!("{:?}",args);
    if let Err(e)=greprs::run(config){
        writeln!((&mut stderr)(),"Application error: {}",e)
            .expect("Could not write to stderr");
        process::exit(1);
    };
    // let mut f=File::open(config.filename)
    //     .expect("file not found");
    // let mut contents=String::new();
    // f.read_to_string(&mut contents)
    //     .expect("something went wrong reading the file");
    // println!("With text:\n{}",contents);
}

