use std::fs::{self, DirEntry, File};
use std::io::ErrorKind;
use std::path::Path;
use xml::reader::{EventReader, XmlEvent};

pub fn extract_files<P, F>(dir_path: P, predicate: F) -> std::io::Result<Vec<DirEntry>>
where
    P: AsRef<Path>,
    F: Fn(&DirEntry) -> bool + Copy,
{
    let dir = fs::read_dir(dir_path)?;
    let file_entries = dir.into_iter().fold(vec![], |mut acc, dir_entry_result| {
        let dir_entry = dir_entry_result.unwrap();
        let file_type = dir_entry.file_type().unwrap();
        if file_type.is_file() {
            if predicate(&dir_entry) {
                acc.push(dir_entry);
            }
        } else {
            acc.append(&mut extract_files(dir_entry.path(), predicate).unwrap());
        }
        acc
    });

    Ok(file_entries)
}

pub fn parse_xml_file<P: AsRef<Path>>(file_path: P) -> std::io::Result<Vec<char>> {
    let file_extension = file_path.as_ref().extension().unwrap();
    if file_extension != "xhtml" {
        return Err(std::io::Error::new(
            ErrorKind::InvalidInput,
            format!(
                "Error: trying to parse xml file but getting {:?}",
                file_extension
            ),
        ));
    }
    let file = File::open(file_path)?;
    let content = EventReader::new(file)
        .into_iter()
        .filter_map(|event_result| match event_result {
            Ok(XmlEvent::Characters(text)) => Some(text + " "),
            _ => None,
        })
        .collect::<String>();

    Ok(content.chars().collect::<Vec<char>>())
}
