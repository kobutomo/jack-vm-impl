mod parser;
mod translator;
use std::{
    env,
    error::Error,
    fs,
    io::{self, BufRead, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();
    let input = args.get(1);
    let output = args.get(2);
    if input == None || output == None {
        eprintln!("need source file path");
        return Ok(());
    }
    let input = input.unwrap();
    let output = output.unwrap();
    // パース
    let reader = io::BufReader::new(fs::File::open(input)?);
    let mut writer = io::BufWriter::new(fs::File::create(output)?);
    for line in reader.lines() {
        let line = line.unwrap();
        let instruction = parser::parse(line);
        if let Some(instruction) = instruction {
            let s = translator::translate(instruction);
            writer.write(s.as_bytes())?;
        }
    }
    writer.flush()?;
    Ok(())
}
