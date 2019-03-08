# Word2Vec Implementation in Rust

A rust implementation of continuous bag of words.

## Create dataset 

+ Download train.csv from: https://www.kaggle.com/c/quora-question-pairs/data
+ Then run: ```python preprocess_quora.pt train.csv quora_processed.txt```

## Learn Embedding

Run the program specifying the input file, the output embedding file and the output dictionary file.

```
>> cargo run learn data/quora/quora_processed.txt data/quora/quora_dict.bin data/quora/quora_embedding.bin

    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/wordvec learn data/quora/quora_processed.txt data/quora/quora_dict.bin data/quora/quora_embedding.bin`


===========================================
Embedder: Continuous Bag Of Words
  Usage: ./embedder CMD INPUT DICT [OUTPUT] [SEARCHTERM] [k]
      CMD:        search | learn
      INPUT:      one line per document with space separated words. 
      OUTPUT:     a binary file starting with the vector dimension follwed by the flattened word vectors. One per hash value.
      SEARCHTERM: searching a word
      k:          number of results
      DICT:       binar file for dictionary
by: Daniel Kohlsdorf
mailto: dkohlsdorf@gmail.com
===========================================
PARAMS: {"min_rate": "0.0001", "win": "5", "n_samples": "8", "step_rate": "0.002", "dim": "100", "epochs": "5", "start_rate": "0.025"}
input:  data/quora/quora_processed.txt
output: data/quora/quora_embedding.bin
dict:   data/quora/quora_dict.bin
===========================================
```

The program will write binary files. All the parameters for continuous bag of words are specified in the ```Word2Vec.toml```

## Search

You can then search the closest words to another word. Some responses are in the SearchResults.md

```
>> cargo run search data/quora/quora_embedding.bin data/quora/quora_dict.bin munich 15
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/wordvec search data/quora/quora_embedding.bin data/quora/quora_dict.bin munich 15`


===========================================
Embedder: Continuous Bag Of Words
  Usage: ./embedder CMD INPUT DICT [OUTPUT] [SEARCHTERM] [k]
      CMD:        search | learn
      INPUT:      one line per document with space separated words. 
      OUTPUT:     a binary file starting with the vector dimension follwed by the flattened word vectors. One per hash value.
      SEARCHTERM: searching a word
      k:          number of results
      DICT:       binar file for dictionary
by: Daniel Kohlsdorf
mailto: dkohlsdorf@gmail.com
===========================================
input:  data/quora/quora_embedding.bin
dict:   data/quora/quora_dict.bin
search: munich
k:      15
===========================================
```

## REFERENCES

+ Word2Vec:  https://arxiv.org/pdf/1310.4546.pdf
