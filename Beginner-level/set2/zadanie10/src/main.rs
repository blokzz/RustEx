use std::fs::File;
use std::io::{self, Write, Read};

fn main() -> io::Result<()> {
    let mut file = File::create("output.txt")?;
    file.write_all(b"test")?;
    let mut file = File::open("output.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    println!("{}", content);

    Ok(())
}
