extern crate getopts;
extern crate sha2;

use getopts::Options;
use std::env;

use std::fs::File;
use std::io::prelude::*;
use std::io;

use sha2::{Sha256, Digest};

pub struct Sha {
    hasher: Sha256
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
        Sha { hasher: Sha256::new() }
    }

    pub fn update(&mut self, buf : &[u8]) {
        self.hasher.input(buf);
    }

    pub fn digest(self) -> Vec<u8> {
        self.hasher.result().to_vec()
    }

    pub fn hexdigest(self) -> String {
        format!("{:x}", self.hasher.result())
    }
}
