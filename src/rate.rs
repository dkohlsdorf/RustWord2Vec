#[derive(Clone)]
pub struct LearningRate {
    pub rate: f64,
    pub min:  f64,
    pub step: f64
}

impl LearningRate {

    pub fn update(&mut self) {
        self.rate = f64::max(self.rate - self.step, self.min)
    }

}