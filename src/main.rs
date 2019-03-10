#[macro_use]
extern crate serde_derive;
extern crate serde;

pub mod cbow;
pub mod dictionary;
pub mod doc;
pub mod document_stream;
pub mod numerics;
pub mod params;
pub mod rate;
pub mod sampler;
pub mod search;
pub mod window;
pub mod word2vec;

use std::collections::HashMap;

fn header() {
    println!("Embedder: Continuous Bag Of Words");
    println!("  Usage: ./embedder CMD INPUT DICT [OUTPUT] [SEARCHTERM] [k]");
    println!("      CMD:        search | learn");
    println!("      INPUT:      one line per document with space separated words. ");
    println!("      OUTPUT:     a binary file starting with the vector dimension followed by the flattened word vectors. One per hash value.");
    println!("      SEARCHTERM: searching a word");
    println!("      k:          number of results");
    println!("      DICT:       binary file for dictionary");
    println!("by: Daniel Kohlsdorf");
    println!("mailto: dkohlsdorf@gmail.com");
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        let cmd      = &args[1];
        let input    = &args[2];
        let dictname = &args[3];

        if cmd.eq("learn") {
            let output = &args[4];
            let mut settings = config::Config::default();
            settings.merge(config::File::with_name("Word2Vec")).unwrap();
            let params = settings.try_into::<HashMap<String, String>>().unwrap();
            let dim = params["dim"].parse::<usize>().unwrap();
            let win = params["win"].parse::<usize>().unwrap();
            let n_samples = params["n_samples"].parse::<usize>().unwrap();
            let epochs = params["epochs"].parse::<usize>().unwrap();
            let rate = rate::LearningRate {
                rate: params["start_rate"].parse::<f64>().unwrap(),
                min: params["min_rate"].parse::<f64>().unwrap(),
                step: params["step_rate"].parse::<f64>().unwrap(),
            };

            println!("\n\n===========================================");
            header();
            println!("===========================================");
            println!("PARAMS: {:?}", params);
            println!("input:  {}", input);
            println!("output: {}", output);
            println!("dict:   {}", dictname);
            println!("===========================================\n\n");

            println!("Build HashDict");
            let dict = dictionary::Dictionary::estimate(input.clone());
            println!("Estimate negative sample distribution from input data");
            let unigram = sampler::Sampler::unigram(
                document_stream::DocumentStream::new(input.clone(), &dict)
                    .map(|document| document.words.clone())
                    .flatten()
                    .collect(),
                dict.n_ids,
            );
            println!("Learning Word2Vec");
            let mut word2vec = word2vec::Word2Vec::new(unigram, &dict, dict.n_ids, dim);
            println!(
                "\t with param size: {} {}",
                word2vec.model.embed.rows(),
                word2vec.model.embed.cols
            );
            word2vec.optimize(input.clone(), win, n_samples, rate, epochs);
            println!("Writing embeddings");
            word2vec.model.embed.write(output.clone());
            println!("Writing dictionary");
            dict.write(dictname.clone());
        } else {
            let token = &args[4];
            let k     = args[5].parse::<usize>().unwrap();
            println!("\n\n===========================================");
            header();
            println!("===========================================");
            println!("input:  {}", input);
            println!("dict:   {}", dictname);
            println!("search: {}", token);
            println!("k:      {}", k);
            println!("===========================================\n\n");
            let s = search::Search::new(input.clone(), dictname.clone());
            println!("Searching: ... ");
            for result in s.search(token, k).iter() {
                println!("{}: {}", result.result, result.distance);
            }
        }
    }
}
