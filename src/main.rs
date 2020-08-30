extern crate getopts;

use getopts::Options;
use std::env;

use std::fs::File;
use std::io::prelude::*;
use std::io;

pub struct Sha {
    hash: [u8; 32]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o", "output", "set output file name", "NAME");
    opts.optopt("i", "input", "set input file name", "NAME");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    let input = matches.opt_str("i").unwrap();
    let h = hash_file(input).unwrap();
    println!("{}", h.hexdigest())
}

fn hash_file(fname: String) -> io::Result<Sha> {
    let mut f = File::open(fname)?;
    let mut h = Sha::new();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    h.update(&buf);
    Ok(h)
}

impl Sha {
    pub fn new() -> Self {
        Sha { hash: [ 0; 32 ] }
    }

    pub fn update(&mut self, buf : &[u8]) {
        self.hash[0] ^= buf[0]
    }

    pub fn digest(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(&self.hash);
        result
    }

    pub fn hexdigest(&self) -> String {
        let d = self.digest();
        let mut r = String::new();
        for i in 0..32 {
            r = r + &format!("{:02x}", d[i]).to_string();
        }
        r
    }
}
