use std::{fs::File, io::{Write, Read, self}, error::Error};

fn write_into_file(content: &str, file_name: &str) -> io::Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(content.as_bytes())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut my_file = File::create("test.txt")?;

    my_file.write_all("lorem ipsum".as_bytes())?;

    let mut my_file = File::open("test.txt")?;

    let mut buf = String::new();

    my_file.read_to_string(&mut buf)?;

    println!("File contents: {}", buf);

    write_into_file("lorem ipsum two", "test2.txt")?;

    Ok(())
}
