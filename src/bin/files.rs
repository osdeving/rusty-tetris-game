use std::{fs::File, io::{Write, Read, self}, error::Error};

fn write_into_file(content: &str, file_name: &str) -> io::Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(content.as_bytes())
}

// using long form of error handler
// fn write_into_file(content: &str, file_name: &str) -> io::Result<()> {
//     let mut file = match File::create(file_name) {
//         Ok(f) => f,
//         Err(e) => return Err(e),
//     };

//     file.write_all(content.as_bytes())
// }



// using try! macro  (deprecated) instead elvis operator (?)
// fn write_into_file(content: &str, file_name: &str) -> io::Result<()> {
//     let mut file = try!(File::create(file_name));
//     file.write_all(content.as_bytes())
// }


fn read_from_file(file_name: &str) -> io::Result<String> {
    let mut content = String::new();
    File::open(file_name)?.read_to_string(&mut content)?;
    Ok(content)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut my_file = File::create("test.txt")?;

    my_file.write_all("lorem ipsum".as_bytes())?;

    let mut my_file = File::open("test.txt")?;

    let mut buf = String::new();

    my_file.read_to_string(&mut buf)?;
    println!("File contents: {}", buf);


    let file_name = "test2.txt";
    write_into_file("lorem ipsum two", file_name)?;

    let content = read_from_file(file_name)?;
    println!("File contents using a separate func: {}", content);

    Ok(())
}
