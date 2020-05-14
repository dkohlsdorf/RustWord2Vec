use crate::window::*;
use crate::numerics::*;
use crate::dictionary::*;

pub struct Document {
    pub words: Vec<usize>,
    pub document_ids: Vec<usize>
}

impl Document {

    pub fn new(document_ids: Vec<String>, words: Vec<String>, dict: &Dictionary) -> Document {        
        Document {
            words: words.iter().map(|word| dict.words2id[word]).collect(),
            document_ids: document_ids.iter().map(|id| dict.words2id[id]).collect()
        }
    }
    
    pub fn window(&self, pos: usize, size: usize) -> Option<Window> {
        let start = sub(pos, size);
        let stop  = usize::min(pos + size, self.words.len() - 1); 
        if stop - start == 2 * size {
            Some(Window {ids: &self.document_ids, words: &self.words[start as usize .. stop as usize], predict_pos: size})
        } else {
            None
        }
    }

}
