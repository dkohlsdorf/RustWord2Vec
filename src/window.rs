
pub struct Window<'a> {
    pub words:       &'a [usize], 
    pub predict_pos: usize
}

impl<'a> Window<'a> {

    pub fn label(&self) -> usize {
        self.words[self.predict_pos]
    }

}