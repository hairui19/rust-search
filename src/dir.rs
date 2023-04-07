use crate::dir;
use crate::Lexer;
use std::collections::HashMap;
use std::fs::{self, DirEntry, File};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use xml::reader::{EventReader, XmlEvent};

fn extract_files<P, F>(dir_path: P, predicate: F) -> std::io::Result<Vec<DirEntry>>
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

fn parse_xml_file<P: AsRef<Path>>(file_path: P) -> std::io::Result<Vec<char>> {
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

pub fn write_xhtml_files_in_json<P, Q>(target_file_path: P, from_dir_path: Q) -> std::io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let file_entries = dir::extract_files(from_dir_path.as_ref(), |dir_entry| {
        match dir_entry.path().extension() {
            Some(extension) => extension == "xhtml",
            None => false,
        }
    })?;

    let mut term_frequency_index = HashMap::<PathBuf, HashMap<String, usize>>::new();
    for entry in file_entries {
        println!("Indexing {:?}", entry.path());
        let content = dir::parse_xml_file(entry.path())?;
        let term_frequency = Lexer::new(&content).into_iter().fold(
            HashMap::<String, usize>::new(),
            |mut tf, token| {
                tf.entry(token.iter().collect())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                tf
            },
        );

        let mut stats = term_frequency.iter().collect::<Vec<_>>();

        stats.sort_by_key(|(_, freq)| *freq);
        stats.reverse();
        term_frequency_index
            .entry(entry.path())
            .or_insert(term_frequency);
    }

    let file = File::create(target_file_path.as_ref())?;
    _ = serde_json::to_writer_pretty(file, &term_frequency_index);

    Ok(())
}
