use std::{io::{BufRead, BufReader, Cursor, Read}};

fn main() {
    let mut buf = [0u8; 10];

    let mut input = BufReader::new(Cursor::new("Input data"));

    input.read(&mut buf).ok();

    print!("Read from a buffered reader: ");

    for byte in buf{
        print!("{}", char::from(byte));
    }

    println!();
}
