use crate::window::*;
use crate::numerics::*;
use crate::dictionary::*;

pub struct Document {
    pub words: Vec<usize>
}

impl Document {

    pub fn new(words: Vec<String>, dict: &Dictionary) -> Document {        
        Document {
            words: words.iter().map(|word| dict.words2id[word]).collect()
        }
    }
    
    pub fn window(&self, pos: usize, size: usize) -> Option<Window> {
        let start = sub(pos, size);
        let stop  = usize::min(pos + size, self.words.len() - 1); 
        if stop - start == 2 * size {
            Some(Window {words: &self.words[start as usize .. stop as usize], predict_pos: size})
        } else {
            None
        }
    }

}
