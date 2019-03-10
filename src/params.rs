extern crate rand;
extern crate bincode;

extern crate serde;

use std::fs::File;
use rand::Rng;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct ParameterStore {
    pub cols: usize,
    pub weights: Vec<f64>
}

impl ParameterStore {

    pub fn seeded(rows: usize, cols: usize) -> ParameterStore {
        let mut rng = rand::thread_rng();
        let mut weights = Vec::new();
        for _i in 0 .. (rows * cols) {
            let uniform = (rng.gen_range(0.0, 1.0) - 0.5) / cols as f64;
            weights.push(uniform);
        }
        ParameterStore { cols, weights }
    }

    pub fn zeros(rows: usize, cols: usize) -> ParameterStore {
        ParameterStore { cols, weights: vec![0.0; rows * cols] }
    }

    pub fn load<P : AsRef<Path>>(filename: P) -> ParameterStore {
        let mut file = File::open(filename).unwrap();
        bincode::deserialize_from(&mut file).unwrap()
    }

    pub fn write<P : AsRef<Path>>(&self, filename: P) {
        let mut fp = File::create(filename).unwrap();
        bincode::serialize_into(&mut fp, self).unwrap();
    }

    pub fn rows(&self) -> usize {
        self.weights.len() / self.cols
    } 

    pub fn at(&self, i: usize) -> &[f64] {
        let from = i * self.cols;
        let to = (i + 1) * self.cols;
        &self.weights[from .. to]
    }

    pub fn update(&mut self, i: usize, v: &[f64]) {
        let from = i * self.cols;
        for (i, x) in v.iter().enumerate().take(self.cols) {
            self.weights[i + from] += x;
        }
    }

    pub fn avg(&self, result: &mut Vec<f64>, indices: Vec<usize>) {
        for (col, x) in result.iter_mut().enumerate().take(self.cols) {
            *x = 0.0;
            for row in indices.iter() {
                let from = row * self.cols;
                *x += self.weights[col + from];
            }
            *x /= indices.len() as f64;
        }
    }
}
