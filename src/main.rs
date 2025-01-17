use chrono::{Duration, NaiveDate};
use clap::Parser;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use core::str;
use std::fs;
use io::Read;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;

#[derive(Clone, Serialize, Deserialize, Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    directory: String,
}

fn main() -> io::Result<()> {
    let cli = Args::parse();
    let dir_path = Path::new(&cli.directory);

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file()
            && !path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .contains("output")
            && path.extension().map_or(false, |ext| ext == "xml")
        {
            let input = path.display().to_string();
            let filename: String = format!("{}", path.file_name().unwrap().to_str().unwrap());
            let output: String = format!("{}\\without24_{}", dir_path.display(), filename);
            match update_file(&input, &output) {
                Err(err) => eprintln!("filename: {} error: {}", dir_path.display(), err),
                Ok(..) => (),
            }
        }
    }
     
    Ok(())
}

fn update_file(input: &str, output: &str) -> io::Result<()> {

    let mut contents = Vec::new();
    let mut file = File::open(&input)?;    
    file.read_to_end(&mut contents)?;

    // Try to interpret the contents as UTF-8
    let text = match str::from_utf8(&contents) {
        Ok(text) => text.to_string(),
        Err(_) => {
            // If it's not valid UTF-8, handle it as lossy UTF-8 (ISO-8859-1 fallback)
            let lossy_text = String::from_utf8_lossy(&contents);
            lossy_text.to_string()
        }
    };

    //let input_file = File::open(&input)?;
    let output_path = Path::new(output);
    //let reader = io::BufReader::new(input_file);
    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&output_path)?;
    let re = Regex::new(r"(\d{4}-\d{2}-\d{2})T24:00:00").unwrap();

    //for line in reader.lines() {
    for line in text.lines() {
        //let mut line = line?;
        let mut line = line.to_string();
        if let Some(cap) = re.captures(&line) {
            if let Some(date_str) = cap.get(1) {
                if let Ok(parsed_date) = NaiveDate::parse_from_str(date_str.as_str(), "%Y-%m-%d") {
                    let new_date = parsed_date + Duration::days(1);
                    let new_date_time = format!("{}T00:00:00", new_date.format("%Y-%m-%d"));
                    line = line.replace(&cap[0], &new_date_time);
                }
            }
        }
        writeln!(output_file, "{}", line)?;
    }
    println!("wrote {} ==> {}", input, output);

    Ok(())
}
