use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

const BANNER_PATH: &str = "./banner.txt";

pub fn dump_banner() -> io::Result<()> {
    println!();
    let path = Path::new(BANNER_PATH);
    let banner_file = File::open(path)?;
    let reader = BufReader::new(banner_file);
    for line in reader.lines() {
        println!("{}", line?);
    }
    println!();
    Ok(())
}
