#![allow(non_snake_case)]

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    //usage ./rustcrack method wordlist hash
    let args: Vec<String> = env::args().collect();
    let enc = &args[1];
    let wordlist = &args[2];
    let input = &args[3];
    if let Ok(lines) = read_lines(wordlist) {
        for line in lines {
            if let Ok(words) = line {
                if enc == "md5" {
                    md5(input.to_string(), words.clone())
                } else if enc == "sha256" {
                    sha256(input.to_string(), words)
                } else if enc == "base64" {
                    base64(input.to_string(), words)
                } else if enc == "md2" {
                    md2(input.to_string(), words)
                } else if enc == "unsure" {
                    unsure(input.to_string(), words)
                } else {
                    println!("Invalid encryption method!")
                }
            }
        }
    }
}

fn unsure(input: String, word: String) {
    md5(input.to_string(), word.clone());
    sha256(input.to_string(), word.clone());
    base64(input.to_string(), word);
}

fn md2(input: String, word: String) {
    use md2::{Md2, Digest};
    let mut hasher = Md2::new();
    hasher.update(word.clone());
    let digest = hasher.finalize();
    let digest: String = format!("{:x}", digest);
    println!("testing: {:?}", digest.to_string());
    if input == digest.to_string() {
        finished(input, word);
    }
}

fn md5(input: String, word: String) {
    let digest = md5::compute(word.clone());
    let digest: String = format!("{:x}", digest);
    println!("testing: {}", digest);
    if input == digest {
        finished(input, word);
    }
}

fn base64(input: String, word: String) {
    use base64::{encode};
    let digest = encode(word.clone());
    println!("testing: {}", digest);
    if input == digest {
        finished(input, word);
    }
}

fn sha256(input: String, word: String) {
    use sha256::digest;
    let digestsha = digest(word.clone());
    println!("testing: {}", digestsha);
    if input == digestsha {
        finished(input, word);
    }
}

fn finished(input: String, word: String) {
    println!("FOUND: {} is {}", input, word);
    std::process::exit(0);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
