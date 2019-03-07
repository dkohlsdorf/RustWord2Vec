use std::collections::BinaryHeap;

use crate::dictionary::Dictionary;
use crate::numerics::*;
use crate::params::ParameterStore;

use std::cmp::Ordering;

#[derive(PartialEq)]
pub struct SearchResult {
    pub result: String,
    pub distance: f64,
}

impl Eq for SearchResult {}

impl Ord for SearchResult {
    fn cmp(&self, other: &SearchResult) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for SearchResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if other.distance < self.distance {
            return Some(Ordering::Less);
        } else if other.distance > self.distance {
            return Some(Ordering::Greater);
        }
        None
    }
}

pub struct Search {
    dict: Dictionary,
    embeddings: ParameterStore,
}

impl Search {

    pub fn new(embed_file: String, dict_file: String) -> Search {
        println!("\tloading embedding");
        let embedding = ParameterStore::load(embed_file);
        println!("\tloading dict");
        let dict = Dictionary::load(dict_file);
        Search {dict : dict, embeddings: embedding}
    }

    pub fn search(&self, token: &String, k: usize) -> Vec<SearchResult> {
        let mut pq = BinaryHeap::new();
        let token = self.dict.words2id[token];
        println!("{}", token);
        let query = self.embeddings.at(token);
        println!("{:?}", query);
        for i in 0..self.embeddings.rows() {
            let distance = euclidean(query, self.embeddings.at(i));
            println!("{}", distance);
            pq.push(SearchResult {
                result: self.dict.id2words[&i].clone(), 
                distance: distance,
            });
        }
        let mut result = Vec::new();
        for _i in 0..k {
            result.push(pq.pop().unwrap());
        }
        result
    }

}
