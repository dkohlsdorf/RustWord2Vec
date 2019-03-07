pub struct LearningRate {
    pub rate: f64,
    pub min:  f64,
    pub step: f64
}

impl LearningRate {

    pub fn update(&self) -> LearningRate {
        LearningRate {
            rate: f64::max(self.rate - self.step, self.min),
            min: self.min,
            step: self.step
        }
    }

}