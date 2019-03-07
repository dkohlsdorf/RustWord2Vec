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
        let mut dict = Dictionary {
            words2id: HashMap::new(),
            id2words: HashMap::new(),
            n_ids: 0,
        };
        let mut reading = true;
        let mut current_id = 0;
        while reading {
            let mut line = String::new();
            let result = reader.read_line(&mut line);
            match result {
                Ok(n) if n > 0 => {
                    let tokens = line.trim().split_terminator(" ").collect::<Vec<_>>();
                    for token in tokens.iter() {
                        let x = String::from(*token);
                        if !dict.words2id.contains_key(&x) {
                            dict.words2id.insert(x.clone(), current_id);
                            dict.id2words.insert(current_id, x.clone());
                            current_id += 1;
                        }
                    }
                }
                _ => {
                    reading = false;
                }
            }
        }
        dict.n_ids = current_id;
        dict
    }

    pub fn load(filename: String) -> Dictionary {
        let mut file = File::open(filename).unwrap();
        println!("hi");
        bincode::deserialize_from(&mut file).unwrap()
    }

    pub fn write(&self, filename: String) {
        let mut fp = File::create(filename).unwrap();
        let mut encoded: Vec<u8> = bincode::serialize(self).unwrap();
        fp.write_all(&mut encoded).expect(&String::from("Could not write binary params file"));
        let _ = fp.flush();
    }

}