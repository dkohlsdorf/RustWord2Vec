use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::dictionary::*;
use crate::doc::*;

pub struct DocumentStream<'a> {
    reader: BufReader<File>,    
    dict: &'a Dictionary
}

impl<'a> Iterator for DocumentStream<'a> {
    type Item = Document;
    fn next(&mut self) -> Option<Document> {
        self.next_document()
    }
}

impl<'a> DocumentStream<'a> {

    pub fn new<P : AsRef<Path>>(filename: P, dict: &Dictionary) -> DocumentStream {
        let fp = File::open(filename).expect("Input file not found");
        DocumentStream { reader: BufReader::new(fp), dict }
    }

    pub fn next_document(&mut self) -> Option<Document> {
        let mut line = String::new();
        let result = self.reader.read_line(&mut line);
        match result {
            Ok(n) if n > 0 => Some(DocumentStream::doc(line, &self.dict)),
            _ => None    
        }
    }

    fn doc(line: String, dict: &Dictionary) -> Document {
        let tokens: Vec<&str> = line.trim().split_terminator(' ').collect();
        let mut words: Vec<String> = Vec::new();
        for token in tokens {            
            words.push(String::from(token));
        }    
        Document::new(words, dict)
    }

}
