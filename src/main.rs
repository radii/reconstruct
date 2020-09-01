extern crate getopts;
extern crate sha2;

use getopts::Options;
use std::env;

use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
use std::io;

use sha2::{Sha256, Digest};

const BUFFER_SIZE : usize = 4096;
const FANOUT : usize = 256;

pub struct HashTree {
    hasher: Sha256,
    h0: Vec<u32>
}

fn main() {
    let args: Vec<String> = env::args().collect();

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

fn short_hash(buf : &[u8]) -> u32 {
    let mut h = Sha256::new();
    h.input(&buf);
    let r = h.result();
    let (int_bytes, _) = r.split_at(std::mem::size_of::<u32>());
    return u32::from_be_bytes(int_bytes.try_into().unwrap())
}

fn hash_file(fname: String) -> io::Result<HashTree> {
    let mut f = File::open(fname)?;
    let mut h = HashTree::new();
    let mut buf = [0u8; BUFFER_SIZE];
    loop {
        let n = match f.read(&mut buf) {
            Ok(n) => { n }
            Err(e) => return Err(e)
        };
        h.update(&buf[..n]);
        if n == 0 || n < BUFFER_SIZE {
            break;
        }
    }
    Ok(h)
}

impl HashTree {
    pub fn new() -> Self {
        HashTree { hasher: Sha256::new(), h0: Vec::new() }
    }

    pub fn update(&mut self, buf : &[u8]) {
        self.hasher.input(buf);
        let h0 = short_hash(buf);
        self.h0.push(h0);
    }

    pub fn digest(self) -> Vec<u8> {
        self.hasher.result().to_vec()
    }

    pub fn hexdigest(self) -> String {
        format!("{:x} {}", self.hasher.result(), self.h0.len())
    }
}
