use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

extern crate bincode;
extern crate rand;
extern crate serde;

#[derive(Serialize, Deserialize)]
pub struct Dictionary {
    pub words2id: HashMap<String, usize>,
    pub id2words: HashMap<usize, String>,
    pub n_ids: usize,
}

impl Dictionary {

    pub fn estimate(input: String) -> Dictionary {
        let fp_in = File::open(input).expect("Input file not found");
        let mut reader = BufReader::new(fp_in);
        let mut dict = Dictionary::default();
        let mut current_id = 0;
        let mut line = String::new();

        while let Ok(n) = reader.read_line(&mut line) {
            if n < 1 { break; }

            for token in line.trim().split_terminator(' ') {
                if !dict.words2id.contains_key(token) {
                    dict.words2id.insert(token.to_string(), current_id);
                    dict.id2words.insert(current_id, token.to_string());
                    current_id += 1;
                }
            }

            line.clear();
        }

        dict.n_ids = current_id;
        dict
    }

    pub fn load(filename: String) -> Dictionary {
        let mut file = File::open(filename).unwrap();
        bincode::deserialize_from(&mut file).expect(&"Could not write binary params file".to_string())
    }

    pub fn write(&self, filename: String) {
        let mut fp = File::create(filename).unwrap();
        bincode::serialize_into(&fp, self).unwrap();
        fp.flush().unwrap();
    }
}

impl Default for Dictionary {
    fn default() -> Dictionary {
        Dictionary {
            words2id: HashMap::new(),
            id2words: HashMap::new(),
            n_ids: 0,
        }
    }
}
