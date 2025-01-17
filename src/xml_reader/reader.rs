use quick_xml::events::{Event, BytesEnd, BytesStart};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use std::fs::File;
use std::io::{BufReader, BufWriter};


pub fn read(path: &str) -> std::io::Result<()> {
    let file = File::open(path)?;
    let reader = Reader::from_reader(BufReader::new(file));
    reader.config_mut().trim_text(true);
    
    println!("{:?}", reader);
    Ok(())
}
