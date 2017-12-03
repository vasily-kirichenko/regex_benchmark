#![feature(universal_impl_trait)]

extern crate time;
extern crate regex;
extern crate itertools;
#[macro_use]
extern crate error_chain;

use time::{PreciseTime, Duration};
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use itertools::*;


mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
            Regex(::regex::Error);
        }
    }
}

use errors::*;

fn tc<T>(f: impl FnOnce() -> Result<T>) -> Result<(T, Duration)> {
    let start = PreciseTime::now();
    let res = f()?;
    Ok((res, start.to(PreciseTime::now())))
}

fn cleanup_name(line: &str) -> String {
    line.split(|x| x == ' ' || x == ';' || x == '"')
        .filter(|x| !x.is_empty())
        .join(".")
}

fn search(file: &str, pattern: &str) -> Result<Vec<String>> {
    let r = Regex::new(pattern)?;
    let f = File::open(file)?;
    let buff = BufReader::new(f);
    let mut res = Vec::new();
    for line in buff.lines().take(10_000_000) {
        let line = line?;
        match r.captures(&line).and_then(|caps| caps.name("name")) {
            Some(name) => res.push(cleanup_name(name.as_str())),
            None => ()
        }
    }
    Ok(res)
}

fn run() -> Result<()> {
    //let pat_ci = r"(?i)\{.*(?P<name>microsoft.*)\|\]";
    let pat_cs = r"\{.*(?P<name>microsoft.*)\|\]";
    let (res, elapsed) = tc(|| {
        search("d:\\big.txt", pat_cs)
    })?;
    println!("Res count = {}, Elapsed {}", res.len(), elapsed);
    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        use error_chain::ChainedError;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";
        writeln!(stderr, "{}", e.display_chain()).expect(errmsg);
        std::process::exit(1);
    }
}

