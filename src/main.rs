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
    let output = output.unwrap();
    let input = input.unwrap();
    let paths = fs::read_dir(input).unwrap();
    let mut writer = io::BufWriter::new(fs::File::create(output)?);
    let mut tl = translator::Translator::new();
    writer.write(tl.generate_boostrap().as_bytes())?;
    for path in paths {
        let path = path.unwrap().path();
        let extension = path.extension().unwrap();
        let filename = path.file_name().unwrap().to_str().unwrap();
        if extension != "vm" {
            continue;
        }
        // パース
        let reader = io::BufReader::new(fs::File::open(&path)?);
        for line in reader.lines() {
            let line = line.unwrap();
            writer.write(("// ".to_owned() + &(line.clone() + "\n")[..]).as_bytes())?;
            let instruction = parser::parse(line);
            if let Some(instruction) = instruction {
                let s = tl.translate(instruction, filename);
                writer.write(s.as_bytes())?;
            }
        }
        writer.flush()?;
    }
    Ok(())
}
