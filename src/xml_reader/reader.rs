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

    /*
    reader.trim_text(true);

    // Open the output XML file
    let mut writer = Writer::new(BufWriter::new(File::create(format!("output_{}", path)?)));

    let mut buf = Vec::new();

    // Loop through all events in the input XML
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Eof) => break, // End of file reached
            Ok(event) => {
                // Write the event to the output XML
                writer.write_event(event)?;
            }
            Err(e) => {
                eprintln!("Error at position {}: {:?}", reader.buffer_position(), e);
                break;
            }
        }
    }
    */
    Ok(())
}
