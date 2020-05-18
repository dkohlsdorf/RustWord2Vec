use std::path::Path;

use crate::cbow::*;
use crate::sampler::*;
use crate::params::*;
use crate::rate::*;
use crate::document_stream::*;
use crate::dictionary::*;

pub struct Word2Vec<'a> {
    pub model: CBOW,
    unigrams: Sampler,
    dict: &'a Dictionary
}

impl<'a> Word2Vec<'a> {

    pub fn new(unigrams: Sampler, dict: &Dictionary, n_rows: usize, n_hidden: usize) -> Word2Vec {
        Word2Vec {
            model: CBOW { 
                embed:   ParameterStore::seeded(n_rows, n_hidden),
                predict: ParameterStore::zeros(n_rows, n_hidden)
            },
            unigrams,
            dict
        }
    }

    pub fn optimize<P : AsRef<Path>>(&mut self, filename: P, win: usize, n_samples: usize, r: LearningRate, epochs: usize) {
        let mut adjusted_rate = r.clone();
        for epoch in 0 .. epochs {
            let stream = DocumentStream::new(&filename, self.dict);
            let mut total_error = 0.0;
            let mut n_windows = 0;
            for document in stream {
                for win_idx in 0 .. document.words.len() { 
                    if let Some(window) = document.window(win_idx, win) {
                        total_error += self.model.negative_sampeling(&window, adjusted_rate.rate, n_samples, &self.unigrams); 
                        n_windows   += 1;
                        if n_windows % 100 == 0 {
                            println!("\t- EPOCH: {} ERROR: {} RATE: {} WINDOWS: {}", epoch, total_error, adjusted_rate.rate, n_windows);
                            total_error = 0.0;
                        }
                    }       
                }
            }
            adjusted_rate.update();
        }
    }

}
