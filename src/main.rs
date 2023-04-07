mod dir;
mod lexer;

use crate::lexer::Lexer;
use std::collections::HashMap;
use std::fs::{self, DirEntry, File};
use std::path::{Path, PathBuf};
use std::process::exit;
use xml::reader::{EventReader, XmlEvent};

const FILE_PATH: &str = "/Users/hairuilin/Documents/docs.gl/gl4/glVertexAttribDivisor.xhtml";
const DIR_PATH: &str = "/Users/hairuilin/Documents/docs.gl";

fn main() -> std::io::Result<()> {
    let file_entries =
        dir::extract_files(DIR_PATH, |dir_entry| match dir_entry.path().extension() {
            Some(extension) => extension == "xhtml",
            None => false,
        })?;

    let mut term_frequency_index = HashMap::<PathBuf, Vec<(String, usize)>>::new();
    for entry in file_entries {
        println!("Indexing {:?}", entry.path());
        let content = dir::parse_xml_file(entry.path())?;
        let mut term_frequency = Lexer::new(&content).into_iter().fold(
            HashMap::<String, usize>::new(),
            |mut tf, token| {
                tf.entry(token.iter().collect())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                tf
            },
        );

        let mut stats = term_frequency.into_iter().collect::<Vec<_>>();

        stats.sort_by_key(|(_, freq)| *freq);
        stats.reverse();
        term_frequency_index.entry(entry.path()).or_insert(stats);
    }

    for (key, value) in term_frequency_index {
        println!("{:?}", key.display());
        for (token, freq) in value.iter().take(10) {
            println!("    {:?} => {:?}", token, freq);
        }
    }

    Ok(())
}
