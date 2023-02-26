use std::{fs::File, io::{Write, Read}, error::Error};



fn main() -> Result<(), Box<dyn Error>> {
    let mut my_file = File::create("test.txt")?;

    my_file.write_all("lorem ipsum".as_bytes())?;

    let mut my_file = File::open("test.txt")?;

    let mut buf = String::new();

    my_file.read_to_string(&mut buf)?;

    println!("File contents: {}", buf);

    Ok(())
}
