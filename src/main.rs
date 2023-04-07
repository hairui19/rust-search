mod dir;
mod lexer;

use crate::lexer::Lexer;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

const DIR_PATH: &str = "/Users/hairuilin/Documents/docs.gl";
const INDEX_JSON_FILE_PATH: &str = "index-pretty.json";

fn main() -> std::io::Result<()> {
    // dir::write_xhtml_files_in_json("index-pretty.json", DIR_PATH)?;
    let file = File::open(INDEX_JSON_FILE_PATH)?;
    let map: HashMap<PathBuf, HashMap<String, usize>> = serde_json::from_reader(file).unwrap();
    map.iter().for_each(|(name, term_freq)| {
        println!("{:?}", name);
        term_freq.iter().for_each(|(token, freq)| {
            println!("    {:?} => {:?}", token, freq);
        });
    });
    Ok(())
}
